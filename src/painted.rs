//! Segments painted for the terminal: every style resolved, every fill stretched.

use std::fmt;

use nu_ansi_term::{AnsiString, AnsiStrings, Style as AnsiStyle};

use crate::segment::Segment;

/// Text drawn in a single style.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    text: String,
    style: AnsiStyle,
}

/// Segments painted into lines of runs.
///
/// Painting resolves `prev_fg` and `prev_bg` against the run to the left, and
/// stretches each line's fills across whatever width the rest of that line
/// leaves. Lines are what line breaks separate, so there is always at least
/// one, and a prompt that ends in a line break ends with an empty line for the
/// cursor to sit on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Painted {
    lines: Vec<Vec<Run>>,
}

impl Painted {
    /// Paints `segments`, stretching fills to `width` when one is known.
    pub fn new(segments: &[Segment], width: Option<usize>) -> Self {
        Self {
            lines: segments
                .split(|segment| matches!(segment, Segment::LineTerm))
                .map(|line| paint_line(line, width))
                .collect(),
        }
    }
}

/// Paints one line. A fill takes an equal share of the width the rest of the
/// line leaves; without a width, or without room, it keeps its natural width.
///
/// A run inherits from its left neighbor on the same line, except that nothing
/// inherits across a fill: the fill resolves against the run before it, and
/// the run after it starts afresh.
fn paint_line(segments: &[Segment], width: Option<usize>) -> Vec<Run> {
    let is_fill = |segment: &&Segment| matches!(segment, Segment::Fill(_));
    let fills = segments.iter().filter(is_fill).count();
    // Measuring text takes a pass over every grapheme, which only a fill needs.
    let fill_width = width.filter(|_| fills > 0).and_then(|width| {
        let used: usize = segments
            .iter()
            .filter(|segment| !is_fill(segment))
            .map(Segment::width_graphemes)
            .sum();
        width
            .checked_sub(used)
            .filter(|&left| left > 0)?
            .checked_div(fills)
    });

    let mut previous: Option<AnsiStyle> = None;
    segments
        .iter()
        .map(|segment| {
            let painted = match segment {
                Segment::Fill(fill) => fill.ansi_string(fill_width, previous.take().as_ref()),
                segment => {
                    let painted = segment.ansi_string(previous.as_ref());
                    previous = Some(*painted.style_ref());
                    painted
                }
            };
            Run {
                style: *painted.style_ref(),
                text: painted.as_str().to_owned(),
            }
        })
        .collect()
}

/// The bytes a terminal draws, with escape sequences collapsed between runs.
impl fmt::Display for Painted {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut strings: Vec<AnsiString> = Vec::new();
        for (index, line) in self.lines.iter().enumerate() {
            if index > 0 {
                strings.push(AnsiString::from("\n"));
            }
            strings.extend(line.iter().map(|run| run.style.paint(run.text.as_str())));
        }
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
        let painted = Painted::new(
            &[
                text("fg:red bg:blue", "a"),
                text("fg:prev_bg bg:prev_fg", "b"),
                text("fg:prev_bg", "c"),
            ],
            None,
        );

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
        let painted = Painted::new(
            &[
                text("red", "a"),
                Segment::LineTerm,
                text("fg:prev_fg", "b"),
                text("green", "c"),
                fill("."),
                text("fg:prev_fg", "d"),
            ],
            None,
        );

        let foregrounds: Vec<_> = painted.lines[1]
            .iter()
            .map(|run| run.style.foreground)
            .collect();
        assert_eq!(vec![None, Some(Color::Green), None, None], foregrounds);
    }

    #[test]
    fn a_fill_inherits_from_the_run_before_it() {
        let painted = Painted::new(
            &[
                text("fg:red bg:blue", "a"),
                Segment::fill(parse_style_string("fg:prev_bg", None), "."),
            ],
            Some(3),
        );

        assert_eq!(Color::Blue.normal(), painted.lines[0][1].style);
    }

    #[test]
    fn fills_share_the_width_their_line_leaves() {
        let painted = Painted::new(
            &[
                text("", "a"),
                fill("."),
                text("", "b"),
                fill("-"),
                text("", "c"),
                Segment::LineTerm,
                text("", "next"),
                fill("."),
            ],
            Some(12),
        );

        assert_eq!("a....b----c\nnext........", painted.to_string());
    }

    #[test]
    fn a_fill_keeps_its_natural_width_without_room_to_stretch() {
        for width in [None, Some(2)] {
            let painted = Painted::new(&[text("", "ab"), fill("-:-")], width);
            assert_eq!("ab-:-", painted.to_string(), "for width {width:?}");
        }
    }

    #[test]
    fn line_breaks_end_lines_and_a_trailing_one_leaves_an_empty_line() {
        assert_eq!(1, Painted::new(&[], None).lines.len());
        assert_eq!(
            2,
            Painted::new(&[text("", "a"), Segment::LineTerm], None)
                .lines
                .len()
        );
    }

    #[test]
    fn escape_sequences_collapse_across_runs_and_reset_before_a_line_break() {
        let painted = Painted::new(
            &[
                text("red", "a"),
                text("red bold", "b"),
                Segment::LineTerm,
                text("red", "c"),
            ],
            None,
        );

        assert_eq!(
            "\u{1b}[31ma\u{1b}[1mb\u{1b}[0m\n\u{1b}[31mc\u{1b}[0m",
            painted.to_string()
        );
    }
}
