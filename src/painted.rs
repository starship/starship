//! Segments painted for the terminal: every style resolved, every fill stretched.

use std::borrow::Cow;
use std::fmt;

use nu_ansi_term::{AnsiString, AnsiStrings, Style as AnsiStyle};
use unicode_segmentation::UnicodeSegmentation;

use crate::context::Shell;
use crate::print::{Grapheme, UnicodeWidthGraphemes};
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

/// Segments painted into lines of runs.
///
/// Painting resolves `prev_fg` and `prev_bg` against the run to the left, and
/// stretches each line's fills across whatever width the rest of that line
/// leaves. Lines are what line breaks separate, so there is always at least
/// one, and a prompt that ends in a line break ends with an empty line for the
/// cursor to sit on.
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

    /// The text a shell's prompt variable holds for this prompt: what the
    /// terminal draws, with each run escaped so that the shell displays it
    /// literally, except where a run is verbatim.
    pub fn escaped_for(&self, shell: Shell) -> String {
        AnsiStrings(&self.ansi_strings(|run| match run.kind {
            Kind::Text | Kind::Fill => Cow::Owned(shell_prompt_escape(&*run.text, shell)),
            Kind::Verbatim => Cow::Borrowed(&run.text),
        }))
        .to_string()
    }

    /// Every run in the style it is drawn in, with a plain line break between
    /// lines, ready for `AnsiStrings` to collapse into the fewest escape codes.
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
    // Measuring text takes a pass over every grapheme, which only a fill needs.
    let fill_width = width.filter(|_| fills > 0).and_then(|width| {
        let used: usize = styled()
            .filter(|&(kind, ..)| kind != Kind::Fill)
            .map(|(.., value)| value.width_graphemes())
            .sum();
        width
            .checked_sub(used)
            .filter(|&left| left > 0)?
            .checked_div(fills)
    });

    let mut previous: Option<AnsiStyle> = None;
    styled()
        .map(|(kind, style, value)| {
            let resolve = |previous: Option<AnsiStyle>| {
                style.map_or_else(AnsiStyle::default, |style| {
                    style.to_ansi_style(previous.as_ref())
                })
            };
            let (style, text) = match kind {
                Kind::Fill => (
                    resolve(previous.take()),
                    fill_width.map_or(Cow::Borrowed(value), |width| stretch(value, width)),
                ),
                Kind::Text | Kind::Verbatim => {
                    let style = resolve(previous);
                    previous = Some(style);
                    (style, Cow::Borrowed(value))
                }
            };
            Run { kind, style, text }
        })
        .collect()
}

/// `pattern` repeated grapheme by grapheme for as long as it fits in `width`,
/// or as it is if it has no width to repeat.
fn stretch(pattern: &str, width: usize) -> Cow<'_, str> {
    if pattern.width_graphemes() == 0 {
        return Cow::Borrowed(pattern);
    }
    Cow::Owned(
        pattern
            .graphemes(true)
            .cycle()
            .scan(0, |used, grapheme| {
                *used += Grapheme(grapheme).width();
                (*used <= width).then_some(grapheme)
            })
            .collect(),
    )
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
        let painted = Painted::new(&segments, None);

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
            assert_eq!(stretched, stretch(pattern, 10), "{pattern}");
        }
    }

    #[test]
    fn a_zero_width_fill_keeps_its_natural_text() {
        assert_eq!("\u{301}", stretch("\u{301}", 8));
    }

    #[test]
    fn a_fill_keeps_its_natural_width_without_room_to_stretch() {
        let segments = [text("", "ab"), fill("-:-")];
        for width in [None, Some(2)] {
            let painted = Painted::new(&segments, width);
            assert_eq!("ab-:-", painted.to_string(), "for width {width:?}");
        }
    }

    #[test]
    fn line_breaks_end_lines_and_a_trailing_one_leaves_an_empty_line() {
        assert_eq!(1, Painted::new(&[], None).lines.len());
        assert_eq!(
            2,
            Painted::new(&[text("", "a"), Segment::LineBreak], None)
                .lines
                .len()
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
    fn text_is_painted_without_copying_it() {
        let segments = [text("red", "a"), fill("-")];
        let painted = Painted::new(&segments, None);

        assert!(
            painted.lines[0]
                .iter()
                .all(|run| matches!(run.text, Cow::Borrowed(_)))
        );
    }

    #[test]
    fn a_shell_receives_every_run_escaped_except_verbatim_ones() {
        let mut segments = Segment::from_text(None, "50% ");
        segments.extend(Segment::verbatim(None, "%~ "));
        segments.push(fill("%"));
        let painted = Painted::new(&segments, Some(10));

        assert_eq!("50% %~ %%%", painted.to_string());
        assert_eq!("50%% %~ %%%%%%", painted.escaped_for(Shell::Zsh));
    }
}
