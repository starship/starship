//! Segments painted for the terminal: every style resolved, every fill stretched.

use std::borrow::Cow;
use std::fmt;

use nu_ansi_term::{AnsiString, AnsiStrings, Style as AnsiStyle};

use crate::context::Shell;
use crate::print::{Grapheme, UnicodeWidthGraphemes, graphemes};
use crate::segment::{Kind, Segment};
use crate::utils::shell_prompt_escape;

/// Text drawn in a single style, borrowed from the segment it was painted
/// from unless it is a stretched fill.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Run<'a> {
    kind: Kind,
    style: AnsiStyle,
    text: Cow<'a, str>,
}

/// Resolved styles and stretched fills, preserving line breaks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Painted<'a> {
    lines: Vec<Vec<Run<'a>>>,
}

impl<'a> Painted<'a> {
    /// Paints `segments`, stretching fills to `width` when one is known.
    pub fn new(segments: &'a [Segment], width: Option<usize>) -> Self {
        Self {
            lines: segments
                .split(|segment| matches!(segment, Segment::LineBreak))
                .map(|line| paint_line(line, width))
                .collect(),
        }
    }

    /// This prompt drawn below an empty line.
    pub fn below_a_line_break(mut self) -> Self {
        self.lines.insert(0, Vec::new());
        self
    }

    /// Escape text and fills for the shell, preserving verbatim runs.
    pub fn escaped_for(&self, shell: Shell) -> String {
        AnsiStrings(&self.ansi_strings(|run| match run.kind {
            Kind::Text | Kind::Fill => Cow::Owned(shell_prompt_escape(&*run.text, shell)),
            Kind::Verbatim => Cow::Borrowed(&run.text),
        }))
        .to_string()
    }

    /// Preserve line breaks while allowing ANSI sequences to collapse.
    fn ansi_strings<'s>(
        &'s self,
        text: impl Fn(&'s Run<'a>) -> Cow<'s, str>,
    ) -> Vec<AnsiString<'s>> {
        let mut strings = Vec::new();
        for (index, line) in self.lines.iter().enumerate() {
            if index > 0 {
                strings.push(AnsiString::from("\n"));
            }
            strings.extend(line.iter().map(|run| run.style.paint(text(run))));
        }
        strings
    }
}

/// Paints one line. Fills share the width the rest of the line leaves, with
/// remainder columns going to the first fills. Without a width, or without
/// room for each fill, they keep their natural width.
///
/// A run inherits from its left neighbor on the same line, except that nothing
/// inherits across a fill: the fill resolves against the run before it, and
/// the run after it starts afresh.
fn paint_line(segments: &[Segment], width: Option<usize>) -> Vec<Run<'_>> {
    let styled = || {
        segments.iter().filter_map(|segment| match segment {
            Segment::Styled { kind, style, value } => Some((*kind, *style, value.as_str())),
            Segment::LineBreak => None,
        })
    };
    let fills = styled().filter(|&(kind, ..)| kind == Kind::Fill).count();
    // Each fill's share of what the rest of the line leaves, if that is known
    // and a column or more: text a shell expands has no width until the shell
    // has expanded it. Measuring text takes a pass over every grapheme, which
    // only a fill needs.
    let allocation = width.filter(|_| fills > 0).and_then(|width| {
        if styled().any(|(kind, ..)| kind == Kind::Verbatim) {
            return None;
        }
        let used = text_width(segments);
        let available = width.checked_sub(used)?;
        Some((available / fills, available % fills)).filter(|&(share, _)| share > 0)
    });

    let mut previous = None;
    let mut fill_index = 0;
    styled()
        .map(|(kind, style, value)| {
            let inherited = if kind == Kind::Fill {
                previous.take()
            } else {
                previous
            };
            let style = style.map_or_else(AnsiStyle::default, |style| {
                style.to_ansi_style(inherited.as_ref())
            });
            if kind != Kind::Fill {
                previous = Some(style);
            }
            let text = match allocation.filter(|_| kind == Kind::Fill) {
                Some((share, remainder)) => {
                    let width = share + usize::from(fill_index < remainder);
                    fill_index += 1;
                    stretch(value, width, style)
                }
                None if kind == Kind::Fill => finish_fill(Cow::Borrowed(value), style),
                None => Cow::Borrowed(value),
            };
            Run { kind, style, text }
        })
        .collect()
}

/// Adjacent text runs can belong to one grapheme despite their different
/// styles. A fill separates them; its neighbors cannot join through it.
fn text_width(segments: &[Segment]) -> usize {
    segments
        .split(|segment| {
            matches!(
                segment,
                Segment::Styled {
                    kind: Kind::Fill,
                    ..
                }
            )
        })
        .map(|group| {
            let mut values = group.iter().filter_map(|segment| match segment {
                Segment::Styled { value, .. } => Some(value.as_str()),
                Segment::LineBreak => None,
            });
            let mut text = Cow::Borrowed(values.next().unwrap_or_default());
            for value in values {
                text.to_mut().push_str(value);
            }
            text.width_graphemes()
        })
        .sum()
}

/// `pattern` repeated across `width` columns: whole repetitions, then one
/// more up to the last grapheme that fits, if that draws anything. Escape
/// sequences among them could leave a hyperlink open or a style other than
/// the fill's; both are reset after them. A pattern without width keeps its
/// natural text.
fn stretch(pattern: &str, width: usize, style: AnsiStyle) -> Cow<'_, str> {
    let pattern_width = pattern.width_graphemes();
    if pattern_width == 0 {
        return finish_fill(Cow::Borrowed(pattern), style);
    }
    let end = graphemes(pattern)
        .scan(width % pattern_width, |room, grapheme| {
            *room = room.checked_sub(Grapheme(grapheme).width())?;
            Some(end_within(pattern, grapheme))
        })
        .last()
        .unwrap_or(0);
    let partial = Some(&pattern[..end]).filter(|partial| partial.width_graphemes() > 0);
    let stretched = pattern.repeat(width / pattern_width) + partial.unwrap_or_default();
    finish_fill(Cow::Owned(stretched), style)
}

/// A fill cannot leave controls active in the run painted after it, whether
/// stretched or kept at its natural width.
fn finish_fill(mut text: Cow<'_, str>, style: AnsiStyle) -> Cow<'_, str> {
    if text.contains('\x1b') {
        if hyperlink_is_open(&text) {
            text.to_mut().push_str(CLOSE_HYPERLINK);
        }
        text.to_mut()
            .push_str(&format!("{RESET}{}", style.prefix()));
    }
    text
}

/// Where `part`, borrowed from `text`, ends within it.
fn end_within(text: &str, part: &str) -> usize {
    part.as_ptr().addr() - text.as_ptr().addr() + part.len()
}

/// What returns a terminal to its default style, whatever came before.
const RESET: &str = "\x1b[0m";

const CLOSE_HYPERLINK: &str = "\x1b]8;;\x1b\\";

/// OSC 8 state after the bytes that actually fit, including complete repeats.
/// The parser handles terminators and unrelated control strings for us.
fn hyperlink_is_open(text: &str) -> bool {
    #[derive(Default)]
    struct Hyperlink(bool);

    impl anstyle_parse::Perform for Hyperlink {
        fn osc_dispatch(&mut self, params: &[&[u8]], _bell_terminated: bool) {
            if params.len() >= 3 && params[0] == b"8" {
                // Semicolons in a URI become further OSC parameters.
                self.0 = !params[2].is_empty() || params.len() > 3;
            }
        }
    }

    let mut parser: anstyle_parse::Parser = Default::default();
    let mut hyperlink = Hyperlink::default();
    for byte in text.bytes() {
        parser.advance(&mut hyperlink, byte);
    }
    hyperlink.0
}

/// The bytes a terminal draws, with escape sequences collapsed between runs.
impl fmt::Display for Painted<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strings = self.ansi_strings(|run| Cow::Borrowed(&run.text));
        write!(formatter, "{}", AnsiStrings(&strings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::parse_style_string;
    use nu_ansi_term::Color;

    fn text(style: &str, value: &str) -> Segment {
        Segment::from_text(parse_style_string(style, None), value).remove(0)
    }

    fn fill(value: &str) -> Segment {
        Segment::fill(None, value)
    }

    #[test]
    fn previous_colors_resolve_against_the_run_to_the_left_as_painted() {
        let segments = [
            text("fg:red bg:blue", "a"),
            text("fg:prev_bg bg:prev_fg", "b"),
            text("fg:prev_bg", "c"),
        ];
        for width in [None, Some(8)] {
            let painted = Painted::new(&segments, width);
            assert_eq!(
                Color::Blue.on(Color::Red),
                painted.lines[0][1].style,
                "the second run swaps the first run's colors"
            );
            assert_eq!(
                Color::Red.normal(),
                painted.lines[0][2].style,
                "the third run takes the background the second was painted with"
            );
        }
    }

    #[test]
    fn nothing_inherits_across_a_line_break_or_a_fill() {
        let segments = [
            text("red", "a"),
            Segment::LineBreak,
            text("fg:prev_fg", "b"),
            text("green", "c"),
            fill("."),
            text("fg:prev_fg", "d"),
        ];
        let painted = Painted::new(&segments, None);

        let foregrounds: Vec<_> = painted.lines[1]
            .iter()
            .map(|run| run.style.foreground)
            .collect();
        assert_eq!(vec![None, Some(Color::Green), None, None], foregrounds);
    }

    #[test]
    fn a_fill_inherits_from_the_run_before_it() {
        let segments = [
            text("fg:red bg:blue", "a"),
            Segment::fill(parse_style_string("fg:prev_bg", None), "."),
        ];
        let painted = Painted::new(&segments, Some(3));

        assert_eq!(Color::Blue.normal(), painted.lines[0][1].style);
    }

    #[test]
    fn fills_share_the_width_their_line_leaves() {
        let segments = [
            text("", "a"),
            fill("."),
            text("", "b"),
            fill("-"),
            text("", "c"),
            Segment::LineBreak,
            text("", "next"),
            fill("."),
        ];
        let painted = Painted::new(&segments, Some(12));

        assert_eq!("a.....b----c\nnext........", painted.to_string());
    }

    #[test]
    fn remainder_columns_go_to_the_first_two_or_three_fills() {
        let two = [
            text("", "a"),
            fill("."),
            text("", "b"),
            fill("-"),
            text("", "c"),
        ];
        let three = [
            text("", "a"),
            fill("."),
            text("", "b"),
            fill("-"),
            text("", "c"),
            fill("_"),
        ];
        for (width, expected) in [
            (10, "a....b---c"),
            (11, "a....b----c"),
            (12, "a.....b----c"),
        ] {
            assert_eq!(expected, Painted::new(&two, Some(width)).to_string());
        }
        for (width, expected) in [
            (10, "a...b--c__"),
            (11, "a...b---c__"),
            (12, "a...b---c___"),
        ] {
            assert_eq!(expected, Painted::new(&three, Some(width)).to_string());
        }
        for width in 6..30 {
            assert_eq!(
                width,
                Painted::new(&two, Some(width))
                    .to_string()
                    .width_graphemes()
            );
            assert_eq!(
                width,
                Painted::new(&three, Some(width))
                    .to_string()
                    .width_graphemes()
            );
        }
    }

    #[test]
    fn fill_layout_measures_graphemes_across_adjacent_styles() {
        let segments = [
            text("red", "👩"),
            text("blue", "‍👩‍"),
            text("green", "👦‍👦"),
            fill("."),
            text("red", "a"),
            text("blue", "\u{301}"),
        ];
        let painted = Painted::new(&segments, Some(8));
        assert_eq!(".....", painted.lines[0][3].text);
        assert_eq!(8, painted.to_string().width_graphemes());
        assert_eq!(Some(Color::Red), painted.lines[0][0].style.foreground);
        assert_eq!(Some(Color::Blue), painted.lines[0][1].style.foreground);
        assert_eq!(Some(Color::Green), painted.lines[0][2].style.foreground);

        let segments = [text("red", "👩"), fill("."), text("blue", "‍👩‍👦‍👦")];
        let painted = Painted::new(&segments, Some(8));
        assert_eq!("....", painted.lines[0][1].text);
        assert_eq!(8, painted.to_string().width_graphemes());
    }

    #[test]
    fn a_fill_repeats_whole_graphemes_that_fit() {
        for (pattern, stretched) in [
            (".", ".........."),
            (".:", ".:.:.:.:.:"),
            ("-:-", "-:--:--:--"),
            ("🟦", "🟦🟦🟦🟦🟦"),
            ("🟢🔵🟡", "🟢🔵🟡🟢🔵"),
        ] {
            assert_eq!(
                stretched,
                stretch(pattern, 10, AnsiStyle::new()),
                "{pattern}"
            );
        }
    }

    /// Every pattern of up to three pieces, among graphemes narrow, wide and
    /// without width, a bell and escape sequences, stretched across every
    /// width up to nine: whole repetitions, then a prefix of one more that
    /// ends on a grapheme and draws something, a column short at most, and the
    /// fill's style restored after any escape sequence.
    #[test]
    fn every_short_pattern_stretches_to_whole_graphemes() {
        const PIECES: [&str; 7] = [
            ".",
            "🟦",
            "\u{301}",
            "\u{7}",
            "\x1b[31m",
            "\x1b[0m",
            "\x1b]8;;x\x1b\\",
        ];
        let style = Color::Green.normal();
        let restore = format!("{RESET}{}", style.prefix());
        let patterns = (1..=3).flat_map(|length| {
            (0..PIECES.len().pow(length)).map(move |mut choice| {
                (0..length)
                    .map(|_| {
                        let piece = PIECES[choice % PIECES.len()];
                        choice /= PIECES.len();
                        piece
                    })
                    .collect::<String>()
            })
        });
        for pattern in patterns {
            let pattern_width = pattern.width_graphemes();
            for width in 0..10 {
                let stretched = stretch(&pattern, width, style);
                let drawn = stretched.strip_suffix(&restore).unwrap_or(&stretched);
                assert_eq!(drawn.contains('\x1b'), drawn.len() < stretched.len());
                assert!(!hyperlink_is_open(&stretched), "{pattern:?} at {width}");
                let drawn = drawn.strip_suffix(CLOSE_HYPERLINK).unwrap_or(drawn);
                if pattern_width == 0 {
                    assert_eq!(pattern, drawn);
                    continue;
                }
                let whole = pattern.repeat(width / pattern_width);
                let partial = drawn.strip_prefix(&whole).unwrap();
                assert!(pattern.starts_with(partial));
                assert!(
                    graphemes(partial)
                        .last()
                        .is_none_or(|last| end_within(partial, last) == partial.len())
                );
                assert!(partial.is_empty() || partial.width_graphemes() > 0);
                let columns = drawn.width_graphemes();
                assert!(
                    columns <= width && columns + 1 >= width,
                    "{pattern:?} at {width}"
                );
            }
        }
    }

    #[test]
    fn a_fill_restores_its_style_after_its_escape_sequences() {
        let segments = [
            Segment::fill(parse_style_string("green", None), "\x1b[31ma🟦"),
            text("green", "NEXT"),
        ];

        assert_eq!(
            "\x1b[32m\x1b[31ma🟦\x1b[31ma\x1b[0m\x1b[32mNEXT\x1b[0m",
            Painted::new(&segments, Some(8)).to_string(),
        );
    }

    #[test]
    fn partial_fills_keep_graphemes_whole_across_controls_and_restore_their_style() {
        let style = Color::Green.normal();
        let restore = format!("{RESET}{}", style.prefix());
        for (grapheme, columns) in [
            ("👩\x1b[31m‍👩‍👦‍👦", 2),
            ("a\x1b[31m\u{301}", 1),
            ("👩\x1b]0;title\x07‍👩‍👦‍👦", 2),
        ] {
            let pattern = format!("{grapheme}🟦");
            assert_eq!("", stretch(&pattern, columns - 1, style));
            assert_eq!(
                format!("{grapheme}{restore}"),
                stretch(&pattern, columns, style)
            );
            assert_eq!(
                format!("{grapheme}{restore}"),
                stretch(&pattern, columns + 1, style)
            );
            let segments = [
                Segment::fill(parse_style_string("green", None), &pattern),
                text("green", "NEXT"),
            ];
            assert_eq!(
                format!("\x1b[32m{grapheme}{restore}NEXT\x1b[0m"),
                Painted::new(&segments, Some(columns + 4)).to_string()
            );
        }
        let pattern = "👩\x07‍👩‍👦‍👦🟦";
        assert_eq!("👩\x07‍👩‍👦‍👦", stretch(pattern, 2, style));
    }

    #[test]
    fn partial_fills_close_hyperlinks_before_the_following_run() {
        let style = Color::Green.normal();
        let restore = format!("{RESET}{}", style.prefix());
        for terminator in ["\x1b\\", "\x07"] {
            for uri in ["https://x", "https://x;a"] {
                let open = format!("\x1b]8;id=fill;{uri}{terminator}");
                let close = format!("\x1b]8;;{terminator}");
                let pattern = format!("{open}a{close}🟦");
                assert_eq!(
                    format!("{open}a{CLOSE_HYPERLINK}{restore}"),
                    stretch(&pattern, 1, style)
                );
                let segments = [
                    Segment::fill(parse_style_string("green", None), &pattern),
                    text("green", "NEXT"),
                ];
                assert_eq!(
                    format!("\x1b[32m{open}a{CLOSE_HYPERLINK}{restore}NEXT{RESET}"),
                    Painted::new(&segments, Some(5)).to_string()
                );
            }
        }
    }

    #[test]
    fn linked_graphemes_stay_whole_when_a_fill_closes_their_hyperlink() {
        let style = Color::Green.normal();
        let open = "\x1b]8;;https://x\x1b\\";
        let family = format!("👩{open}‍👩‍👦‍👦");
        let pattern = format!("{family}{CLOSE_HYPERLINK}🟦");
        assert_eq!("", stretch(&pattern, 1, style));
        assert_eq!(
            format!("{family}{CLOSE_HYPERLINK}{RESET}{}", style.prefix()),
            stretch(&pattern, 2, style)
        );
    }

    #[test]
    fn fill_cleanup_only_closes_hyperlinks_left_open_by_emitted_bytes() {
        let style = Color::Green.normal();
        let restore = format!("{RESET}{}", style.prefix());
        let open = "\x1b]8;;https://x\x1b\\";
        let closed = format!("{open}a{CLOSE_HYPERLINK}");
        assert_eq!(
            format!("{closed}{closed}{restore}"),
            stretch(&closed, 2, style)
        );
        let future = format!("\x1b[31ma{open}🟦{CLOSE_HYPERLINK}");
        assert_eq!(format!("\x1b[31ma{restore}"), stretch(&future, 1, style));
        let title = "\x1b]0;title\x07a🟦";
        assert_eq!(
            format!("\x1b]0;title\x07a{restore}"),
            stretch(title, 1, style)
        );
    }

    #[test]
    fn zero_width_and_natural_fills_restore_controls_before_the_following_run() {
        let open = "\x1b]8;;https://x\x1b\\";
        for (pattern, expected, columns) in [
            ("\x1b[31m", "\x1b[31m".to_owned(), 0),
            (open, format!("{open}{CLOSE_HYPERLINK}"), 0),
            ("\x1b[31ma", "\x1b[31ma".to_owned(), 1),
            (
                "\x1b]8;;https://x\x1b\\a",
                format!("{open}a{CLOSE_HYPERLINK}"),
                1,
            ),
        ] {
            let segments = [
                Segment::fill(parse_style_string("green", None), pattern),
                text("green", "NEXT"),
            ];
            for width in [None, Some(4), Some(4 + columns)] {
                assert_eq!(
                    format!("\x1b[32m{expected}{RESET}\x1b[32mNEXT{RESET}"),
                    Painted::new(&segments, width).to_string(),
                    "{pattern:?} at {width:?}"
                );
            }
            if columns == 0 {
                assert_eq!(
                    format!("\x1b[32m{expected}{RESET}\x1b[32mNEXT{RESET}"),
                    Painted::new(&segments, Some(8)).to_string()
                );
            }
        }
        for pattern in ["", "\u{301}", "\x07"] {
            assert!(matches!(
                finish_fill(Cow::Borrowed(pattern), Color::Green.normal()),
                Cow::Borrowed(_)
            ));
        }
    }

    #[test]
    fn a_fill_keeps_its_natural_width_without_room_to_stretch() {
        let segments = [text("", "ab"), fill("-:-")];
        for width in [None, Some(2)] {
            let painted = Painted::new(&segments, width);
            assert_eq!("ab-:-", painted.to_string(), "for width {width:?}");
        }
        let segments = [text("", "ab"), fill("-:-"), fill(".")];
        assert_eq!("ab-:-.", Painted::new(&segments, Some(3)).to_string());
        assert_eq!("ab-.", Painted::new(&segments, Some(4)).to_string());
    }

    #[test]
    fn trailing_line_breaks_survive_painting() {
        assert_eq!("", Painted::new(&[], None).to_string());
        assert_eq!(
            "a\n",
            Painted::new(&[text("", "a"), Segment::LineBreak], None).to_string()
        );
    }

    #[test]
    fn escape_sequences_collapse_across_runs_and_reset_before_a_line_break() {
        let segments = [
            text("red", "a"),
            text("red bold", "b"),
            Segment::LineBreak,
            text("red", "c"),
        ];

        assert_eq!(
            "\u{1b}[31ma\u{1b}[1mb\u{1b}[0m\n\u{1b}[31mc\u{1b}[0m",
            Painted::new(&segments, None).to_string()
        );
    }

    #[test]
    fn a_shell_receives_every_run_escaped_except_verbatim_ones() {
        let mut segments = Segment::from_text(None, "50% ");
        segments.extend(Segment::verbatim(None, "%~ "));
        segments.push(fill("%"));
        let painted = Painted::new(&segments, Some(10));

        assert_eq!("50% %~ %", painted.to_string());
        assert_eq!("50%% %~ %%", painted.escaped_for(Shell::Zsh));

        let segments = [text("", "50% "), fill("%")];
        let painted = Painted::new(&segments, Some(7));
        assert_eq!("50% %%%", painted.to_string());
        assert_eq!("50%% %%%%%%", painted.escaped_for(Shell::Zsh));
    }

    #[test]
    fn verbatim_text_leaves_fill_width_unknown_only_on_its_own_line() {
        let mut segments = Segment::verbatim(None, "$HOME");
        segments.push(fill("."));
        segments.push(Segment::LineBreak);
        segments.extend(Segment::from_text(None, "x"));
        segments.push(fill("."));
        let painted = Painted::new(&segments, Some(8));

        assert_eq!("$HOME.\nx.......", painted.to_string());
        assert_eq!("$HOME.\nx.......", painted.escaped_for(Shell::Bash));
    }
}
