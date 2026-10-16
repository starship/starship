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

/// Paints one line. A fill takes an equal share of the width the rest of the
/// line leaves; without a width, or without room, it keeps its natural width.
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
    let share = width.filter(|_| fills > 0).and_then(|width| {
        let used: usize = styled()
            .filter(|&(kind, ..)| kind != Kind::Fill)
            .map(|(kind, _, value)| (kind != Kind::Verbatim).then(|| value.width_graphemes()))
            .sum::<Option<usize>>()?;
        Some(width.checked_sub(used)? / fills).filter(|&share| share > 0)
    });

    let mut previous = None;
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
            let text = match share.filter(|_| kind == Kind::Fill) {
                Some(share) => stretch(value, share, style),
                None => Cow::Borrowed(value),
            };
            Run { kind, style, text }
        })
        .collect()
}

/// `pattern` repeated across `width` columns: whole repetitions, then one
/// more up to the last grapheme that fits, if that draws anything. Escape
/// sequences among them could leave a style other than the fill's, which is
/// restored after them. A pattern without width keeps its natural text.
fn stretch(pattern: &str, width: usize, style: AnsiStyle) -> Cow<'_, str> {
    let pattern_width = pattern.width_graphemes();
    if pattern_width == 0 {
        return Cow::Borrowed(pattern);
    }
    let end = graphemes(pattern)
        .scan(width % pattern_width, |room, grapheme| {
            *room = room.checked_sub(Grapheme(grapheme).width())?;
            Some(end_within(pattern, grapheme))
        })
        .last()
        .unwrap_or(0);
    let partial = Some(&pattern[..end]).filter(|partial| partial.width_graphemes() > 0);
    let mut stretched = pattern.repeat(width / pattern_width) + partial.unwrap_or_default();
    if stretched.contains('\x1b') {
        stretched += &format!("{RESET}{}", style.prefix());
    }
    Cow::Owned(stretched)
}

/// Where `part`, borrowed from `text`, ends within it.
fn end_within(text: &str, part: &str) -> usize {
    part.as_ptr().addr() - text.as_ptr().addr() + part.len()
}

/// What returns a terminal to its default style, whatever came before.
const RESET: &str = "\x1b[0m";

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

        assert_eq!("a....b----c\nnext........", painted.to_string());
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
                if pattern_width == 0 {
                    assert_eq!(pattern, stretched);
                    continue;
                }
                let drawn = stretched.strip_suffix(&restore).unwrap_or(&stretched);
                assert_eq!(drawn.contains('\x1b'), drawn.len() < stretched.len());
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
