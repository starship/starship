//! Segments painted for the terminal: every style resolved, every fill stretched.

use std::borrow::Cow;
use std::fmt;

use nu_ansi_term::{AnsiString, AnsiStrings, Style as AnsiStyle};
use unicode_segmentation::UnicodeSegmentation;

use crate::config::Style;
use crate::print::{UnicodeWidthGraphemes, printable};
use crate::segment::{Kind, Segment};

/// Text in one style, borrowed from its segment unless it is a stretched fill.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Run<'a> {
    style: AnsiStyle,
    text: Cow<'a, str>,
}

/// Resolved styles and stretched fills, preserving line breaks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Painted<'a> {
    /// The runs of each line. Lines stay apart because a run inherits its
    /// style, and a fill takes its width, from its own line only.
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

    /// Preserve line breaks while allowing ANSI sequences to collapse.
    fn ansi_strings<'s>(
        &'s self,
        text: impl Fn(&'s Run<'a>) -> Cow<'s, str>,
    ) -> Vec<AnsiString<'s>> {
        let runs: usize = self.lines.iter().map(Vec::len).sum();
        let mut strings = Vec::with_capacity(runs + self.lines.len().saturating_sub(1));
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
/// A run inherits from its left neighbor on the same line, except that
/// nothing inherits across a fill: the fill resolves against the run before
/// it, and the run after it starts afresh.
fn paint_line(segments: &[Segment], width: Option<usize>) -> Vec<Run<'_>> {
    let fills = segments
        .iter()
        .filter(|segment| {
            matches!(
                segment,
                Segment::Styled {
                    kind: Kind::Fill,
                    ..
                }
            )
        })
        .count();
    // How wide each fill is in turn, if each gets a column or more of what
    // the rest of the line leaves. Measuring text takes a pass over every
    // grapheme, which only a fill needs.
    let mut widths = width.filter(|_| fills > 0).and_then(|width| {
        let available = width.checked_sub(text_width(segments))?;
        let share = available / fills;
        (share > 0)
            .then(|| (0..fills).map(move |fill| share + usize::from(fill < available % fills)))
    });

    let mut previous = None;
    let mut runs = Vec::with_capacity(segments.len());
    for segment in segments {
        let Segment::Styled { kind, style, value } = segment else {
            continue;
        };
        match kind {
            Kind::Text => {
                let style = resolve(*style, previous.as_ref());
                previous = Some(style);
                runs.push(Run {
                    style,
                    text: Cow::Borrowed(value),
                });
            }
            Kind::Fill => {
                let pattern = printable(value);
                let text = match widths.as_mut().and_then(Iterator::next) {
                    Some(width) => stretch(pattern, width),
                    None => pattern,
                };
                paint_fill(&mut runs, *style, previous.take(), text);
            }
        }
    }
    runs
}

/// `style` resolved against the style `inherited` from the run before it.
fn resolve(style: Option<Style>, inherited: Option<&AnsiStyle>) -> AnsiStyle {
    style.map_or_else(AnsiStyle::default, |style| style.to_ansi_style(inherited))
}

/// Adds the runs of a fill drawing `text`. A style that refers to the previous
/// colors is resolved for each grapheme against the one before it, so that
/// `fg:prev_bg bg:prev_fg` alternates colors in stripes.
fn paint_fill<'a>(
    runs: &mut Vec<Run<'a>>,
    style: Option<Style>,
    inherited: Option<AnsiStyle>,
    text: Cow<'a, str>,
) {
    if !style.is_some_and(|style| style.references_previous()) {
        runs.push(Run {
            style: resolve(style, inherited.as_ref()),
            text,
        });
        return;
    }
    let mut previous = inherited;
    for grapheme in text.graphemes(true) {
        let style = resolve(style, previous.as_ref());
        previous = Some(style);
        runs.push(Run {
            style,
            text: Cow::Owned(grapheme.to_owned()),
        });
    }
}

/// The width of the text of `segments` between fills. Adjacent text runs can
/// belong to one grapheme despite their different styles, which a fill
/// separates.
fn text_width(segments: &[Segment]) -> usize {
    let mut group = String::new();
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
        .map(|between| {
            group.clear();
            group.extend(between.iter().map(Segment::value));
            group.width_graphemes()
        })
        .sum()
}

/// `pattern` repeated across `width` columns: whole repetitions, then one
/// more up to the last grapheme that fits, if that draws anything. A pattern
/// without width keeps its natural text, as does one whose repetitions would
/// join into fewer graphemes, such as one ending in a joiner.
fn stretch(pattern: Cow<'_, str>, width: usize) -> Cow<'_, str> {
    let pattern_width = pattern.width_graphemes();
    if pattern_width == 0 || pattern.repeat(2).width_graphemes() != 2 * pattern_width {
        return pattern;
    }
    let mut room = width % pattern_width;
    let partial: String = pattern
        .graphemes(true)
        .map_while(|grapheme| {
            room = room.checked_sub(grapheme.width_graphemes())?;
            Some(grapheme)
        })
        .collect();
    let whole = pattern.repeat(width / pattern_width);
    Cow::Owned(match partial.width_graphemes() {
        0 => whole,
        _ => whole + &partial,
    })
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
    use proptest::prelude::*;

    fn text(style: &str, value: &str) -> Segment {
        Segment::from_text(parse_style_string(style, None), value).remove(0)
    }

    fn fill(value: &str) -> Segment {
        Segment::fill(None, value)
    }

    #[test]
    fn a_run_resolves_its_style_against_the_one_to_its_left_on_its_line() {
        let segments = [
            text("fg:red bg:blue", "a"),
            text("fg:prev_bg bg:prev_fg", "b"),
            text("fg:prev_bg", "c"),
            Segment::fill(parse_style_string("fg:prev_bg", None), "."),
            text("fg:prev_fg", "d"),
            Segment::LineBreak,
            text("fg:prev_fg", "e"),
        ];
        let painted = Painted::new(&segments, Some(5));
        let styles: Vec<_> = painted
            .lines
            .iter()
            .flatten()
            .map(|run| run.style)
            .collect();

        assert_eq!(
            vec![
                Color::Red.on(Color::Blue),
                Color::Blue.on(Color::Red),
                Color::Red.normal(),
                AnsiStyle::default(),
                AnsiStyle::default(),
                AnsiStyle::default(),
            ],
            styles,
            "a run takes what the run before it was painted with, a fill takes \
             what came before it, and nothing goes past a fill or a line break"
        );
    }

    #[test]
    fn styles_change_as_little_as_they_can_and_reset_before_a_line_break() {
        assert_eq!("", Painted::new(&[], None).to_string());
        let segments = [
            text("red", "a"),
            text("red bold", "b"),
            Segment::LineBreak,
            text("red", "c"),
            Segment::LineBreak,
        ];

        assert_eq!(
            "\x1b[31ma\x1b[1mb\x1b[0m\n\x1b[31mc\x1b[0m\n",
            Painted::new(&segments, None).to_string()
        );
    }

    #[test]
    fn text_is_measured_as_drawn_whatever_styles_split_its_graphemes() {
        // A family of four, painted in three styles, is drawn two columns wide.
        let segments = [
            text("red", "\u{1f469}"),
            text("blue", "\u{200d}\u{1f469}\u{200d}"),
            text("green", "\u{1f466}\u{200d}\u{1f466}"),
            fill("."),
        ];
        let painted = Painted::new(&segments, Some(8));

        assert_eq!("......", painted.lines[0][3].text);
    }

    #[test]
    fn a_fill_draws_the_text_of_its_pattern_in_its_own_style() {
        let pattern = "\x1b[31m-\x1b]8;;https://x\x1b\\:\x1b]8;;\x07\x07";
        let segments = [
            Segment::fill(parse_style_string("green", None), pattern),
            text("green", "NEXT"),
        ];

        for (width, expected) in [(None, "-:"), (Some(9), "-:-:-")] {
            assert_eq!(
                format!("\x1b[32m{expected}NEXT\x1b[0m"),
                Painted::new(&segments, width).to_string()
            );
        }
    }

    #[test]
    fn a_fill_referring_to_previous_colors_draws_stripes() {
        let segments = [
            text("fg:red bg:blue", "a"),
            Segment::fill(parse_style_string("fg:prev_bg bg:prev_fg", None), ".:"),
            text("fg:prev_fg", "b"),
        ];

        for (width, expected) in [(None, 2), (Some(8), 6)] {
            let painted = Painted::new(&segments, width);
            let stripes: Vec<_> = painted.lines[0][1..=expected]
                .iter()
                .map(|run| (run.text.as_ref(), run.style))
                .collect();

            let swapped = Color::Blue.on(Color::Red);
            let original = Color::Red.on(Color::Blue);
            assert_eq!(
                [
                    (".", swapped),
                    (":", original),
                    (".", swapped),
                    (":", original),
                    (".", swapped),
                    (":", original),
                ][..expected],
                stripes[..],
                "each symbol swaps the colors of the one before it"
            );
            assert_eq!(
                AnsiStyle::default(),
                painted.lines[0][expected + 1].style,
                "nothing inherits across a fill"
            );
        }
    }

    #[test]
    fn a_fill_without_references_is_one_run() {
        let segments = [
            text("red", "a"),
            Segment::fill(parse_style_string("green", None), ".:"),
        ];
        let painted = Painted::new(&segments, Some(5));

        assert_eq!(2, painted.lines[0].len());
        assert_eq!(".:.:", painted.lines[0][1].text);
        assert_eq!(Color::Green.normal(), painted.lines[0][1].style);
    }

    #[test]
    fn a_fill_never_cuts_a_grapheme_however_wide() {
        // One grapheme, four columns wide: ten columns hold two of it.
        let pattern = "\u{1f7e6}\u{301}\u{200d}\u{1f469}";
        let segments = [fill(pattern)];

        assert_eq!(
            pattern.repeat(2),
            Painted::new(&segments, Some(10)).lines[0][0].text
        );
    }

    /// Pieces of text that test graphemes and controls: narrow, wide, joined
    /// and combining, and a joiner with what it joins, so that graphemes
    /// often span pieces; a bell; styles; hyperlinks opened and closed with
    /// either terminator, with and without parameters; and a title.
    const PIECES: [&str; 15] = [
        "a",
        ".",
        "🟦",
        "👩",
        "\u{200d}",
        "\u{200d}\u{1f469}",
        "\u{301}",
        "\u{7}",
        "\x1b[31m",
        "\x1b[0m",
        "\x1b]8;;x\x1b\\",
        "\x1b]8;id=a;https://x;y\x07",
        "\x1b]8;;\x1b\\",
        "\x1b]8;;\x07",
        "\x1b]0;title\x07",
    ];

    fn pieces(count: std::ops::Range<usize>) -> impl Strategy<Value = String> {
        prop::collection::vec(prop::sample::select(&PIECES[..]), count)
            .prop_map(|pieces| pieces.concat())
    }

    proptest! {
        /// A fill draws whole repetitions of the text of its pattern, then a
        /// prefix of one more that ends on a grapheme and draws something,
        /// short of the width by less than the next grapheme, which would not
        /// fit; or else that text as it is: without a width to fill, or when it
        /// has no width, or when its repetitions would join into fewer
        /// graphemes.
        #[test]
        fn a_fill_draws_whole_graphemes_of_its_text(
            pattern in pieces(0..6),
            width in prop::option::of(0_usize..20),
        ) {
            let segments = [fill(&pattern)];
            let painted = Painted::new(&segments, width);
            let drawn = &painted.lines[0][0].text;

            let text = printable(&pattern);
            let columns = text.width_graphemes();
            let additive = text.repeat(2).width_graphemes() == 2 * columns;
            let Some(width) = width.filter(|&width| width > 0 && columns > 0 && additive) else {
                prop_assert_eq!(&text, drawn);
                return Ok(());
            };
            let partial = drawn.strip_prefix(&text.repeat(width / columns));
            let partial = partial.expect("whole repetitions first");
            let boundaries = text.grapheme_indices(true).map(|(index, _)| index);
            prop_assert!(text.starts_with(partial));
            prop_assert!(boundaries.chain([text.len()]).any(|end| end == partial.len()));
            prop_assert!(partial.is_empty() || partial.width_graphemes() > 0);
            let next = text[partial.len()..]
                .graphemes(true)
                .map(|grapheme| grapheme.width_graphemes())
                .find(|&wide| wide > 0);
            let drawn = drawn.width_graphemes();
            prop_assert!(drawn <= width, "{} columns of {}", drawn, width);
            prop_assert!(drawn == width || next.is_some_and(|next| drawn + next > width), "{} columns of {}", drawn, width);
        }

        /// Fills share exactly what the rest of their line leaves, so that it
        /// is as wide as the terminal, the first of them taking a column more
        /// until none is left over; and without a column for each, every fill
        /// keeps its natural width. Text between them is measured whole,
        /// whatever styles split it.
        #[test]
        fn fills_share_what_their_line_leaves(
            line in prop::collection::vec((pieces(0..4), any::<bool>()), 1..6),
            width in 0_usize..40,
        ) {
            let mut segments = Vec::new();
            let mut patterns = Vec::new();
            for (index, (shown, is_fill)) in line.iter().enumerate() {
                if *is_fill {
                    let pattern = [".", "-", ":"][index % 3];
                    patterns.push(pattern);
                    segments.push(fill(pattern));
                } else {
                    let style = ["red", "blue"][index % 2];
                    segments.extend(Segment::from_text(parse_style_string(style, None), shown.as_str()));
                }
            }
            // Each pattern is a column wide.
            let fills = patterns.len();
            let natural = Painted::new(&segments, None).to_string().width_graphemes();
            let room = (fills > 0 && width >= natural).then(|| width - natural + fills);

            let painted = Painted::new(&segments, Some(width));
            let drawn: Vec<String> = segments
                .iter()
                .zip(&painted.lines[0])
                .filter(|(segment, _)| matches!(segment, Segment::Styled { kind: Kind::Fill, .. }))
                .map(|(_, run)| run.text.to_string())
                .collect();
            let expected: Vec<String> = patterns
                .iter()
                .enumerate()
                .map(|(index, pattern)| match room {
                    Some(room) => pattern.repeat(room / fills + usize::from(index < room % fills)),
                    None => (*pattern).to_owned(),
                })
                .collect();
            prop_assert_eq!(expected, drawn);
            if room.is_some() {
                prop_assert_eq!(width, painted.to_string().width_graphemes());
            }
        }
    }
}
