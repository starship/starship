//! A painted prompt.

use std::fmt;
use std::ops::Range;

use nu_ansi_term::{Color, Style as AnsiStyle, ansi::RESET};

use crate::config::Style;
use crate::segment::{FillSegment, Segment};

/// A width, measured in terminal cells, that a prompt line may occupy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TerminalWidth(pub usize);

/// The position of a visual line within a [`Painted`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineIndex(pub usize);

/// A style in which every symbolic colour reference has already been replaced by
/// a concrete colour.
///
/// The wrapped [`nu_ansi_term::Style`] has no representation for the `prev_fg`
/// and `prev_bg` specifiers, and the field is private, so the only way to obtain
/// a `ResolvedStyle` is [`ResolvedStyle::resolve`] — which demands the style of
/// the left neighbour at the same time. A `ResolvedStyle` that still holds a
/// symbolic reference is therefore not constructible.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ResolvedStyle(AnsiStyle);

impl ResolvedStyle {
    /// The style that emits no escape sequences at all.
    pub fn plain() -> Self {
        Self(AnsiStyle::new())
    }

    /// Resolves a symbolic style against the already resolved style of the run
    /// to its left.
    ///
    /// `previous` is `None` wherever there is no left neighbour to inherit from:
    /// at the start of a visual line, and immediately after a fill.
    pub fn resolve(symbolic: Option<Style>, previous: Option<Self>) -> Self {
        match symbolic {
            Some(symbolic) => {
                Self(symbolic.to_ansi_style(previous.map(|Self(style)| style).as_ref()))
            }
            None => Self::plain(),
        }
    }

    /// Whether this style is the absence of styling.
    pub fn is_plain(&self) -> bool {
        self.0.is_plain()
    }

    /// The concrete terminal style that this resolves to.
    pub fn as_ansi_style(&self) -> AnsiStyle {
        self.0
    }
}

/// What produced a [`Run`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunKind {
    /// Literal text emitted by a module.
    Text,
    /// Padding produced by expanding a [`Segment::Fill`] to its final width.
    Fill,
    /// The structural end of a visual line. Newlines are never part of a
    /// [`RunKind::Text`] run.
    LineTerminator,
}

/// A contiguous piece of text that carries a single fully resolved style.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    text: String,
    style: ResolvedStyle,
    kind: RunKind,
}

impl Run {
    /// The literal text of this run. A [`RunKind::LineTerminator`] run is
    /// exactly one newline, and a [`RunKind::Text`] run never contains one.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The style this run is drawn with.
    pub fn style(&self) -> ResolvedStyle {
        self.style
    }

    /// What produced this run.
    pub fn kind(&self) -> RunKind {
        self.kind
    }
}

/// A prompt whose styles are all resolved and whose fills are all expanded.
///
/// The runs are stored in output order. `lines` partitions them into visual
/// lines, so a caller can ask which runs belong to which line without scanning
/// for newlines again.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Painted {
    runs: Vec<Run>,
    lines: Vec<Range<usize>>,
}

impl Painted {
    /// Pass 2: resolves `segments` into their final painted form.
    ///
    /// `available_width` is the width that each visual line may occupy. When it
    /// is `None`, or when a line is already at least that wide, fills are
    /// emitted at their natural width rather than being stretched.
    pub fn paint(segments: &[Segment], available_width: Option<TerminalWidth>) -> Self {
        let mut runs: Vec<Run> = Vec::new();
        let mut lines: Vec<Range<usize>> = Vec::new();

        let mut next_segment = 0;
        while next_segment < segments.len() {
            if next_segment + 1 == segments.len()
                && matches!(segments.get(next_segment), Some(Segment::Text(text)) if text.is_empty())
                && matches!(runs.last(), Some(run) if run.kind() == RunKind::LineTerminator)
            {
                break;
            }

            let line_start = runs.len();
            next_segment = paint_line(segments, next_segment, available_width, &mut runs);
            lines.push(line_start..runs.len());
        }

        Self { runs, lines }
    }

    /// Every run, in output order.
    pub fn runs(&self) -> &[Run] {
        &self.runs
    }

    /// Whether this prompt has no runs at all.
    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
    }

    /// How many visual lines this prompt occupies.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    /// The runs belonging to one visual line, or `None` if the line does not
    /// exist.
    pub fn line(&self, line: LineIndex) -> Option<&[Run]> {
        self.line_range(line).map(|range| &self.runs[range])
    }

    /// The range of [`Painted::runs`] belonging to one visual line, or `None` if
    /// the line does not exist.
    pub fn line_range(&self, line: LineIndex) -> Option<Range<usize>> {
        self.lines.get(line.0).cloned()
    }

    /// Every visual line, in output order.
    pub fn lines(&self) -> impl ExactSizeIterator<Item = &[Run]> {
        self.lines.iter().map(|range| &self.runs[range.clone()])
    }

    /// Pass 3: the exact bytes to write to the terminal, with the escape
    /// sequences of adjacent runs collapsed down to the attributes that change.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    /// Renders the prompt as legible markup rather than escape sequences, in the
    /// spirit of starship's own format strings — for example
    /// `via [ v12.0.0 ](green bold)`.
    ///
    /// Styled runs are wrapped in `[text](style)` and unstyled runs are written
    /// bare; `\`, `[` and `]` in the text are backslash-escaped. Line
    /// terminators appear as real newlines. This is a diagnostic form meant for
    /// reading and for snapshot tests: a few styles that the configuration
    /// parser cannot produce have no format-string spelling, and are rendered
    /// with the closest descriptive token instead.
    pub fn to_markup(&self) -> String {
        let mut markup = String::new();
        for run in &self.runs {
            let text = escape_markup_text(&run.text);
            if run.style.is_plain() {
                markup.push_str(&text);
            } else {
                markup.push('[');
                markup.push_str(&text);
                markup.push_str("](");
                markup.push_str(&style_markup(run.style.0));
                markup.push(')');
            }
        }
        markup
    }
}

/// Writes the prompt as the bytes a terminal should receive.
impl fmt::Display for Painted {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut remaining = self.runs.iter();
        let Some(first) = remaining.next() else {
            return Ok(());
        };

        write!(formatter, "{}", first.style.0.prefix())?;
        formatter.write_str(&first.text)?;

        let mut previous = first;
        for run in remaining {
            match StyleTransition::between(previous.style.0, run.style.0) {
                StyleTransition::AddAttributes(added) => write!(formatter, "{}", added.prefix())?,
                StyleTransition::ResetFirst => {
                    write!(formatter, "{RESET}{}", run.style.0.prefix())?;
                }
                StyleTransition::Nothing => {}
            }
            formatter.write_str(&run.text)?;
            previous = run;
        }

        // The trailing reset is only needed when the last run left attributes
        // switched on.
        if !previous.style.is_plain() {
            formatter.write_str(RESET)?;
        }

        Ok(())
    }
}

/// Writes the prompt as legible markup; see [`Painted::to_markup`].
impl fmt::Debug for Painted {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.to_markup())
    }
}

/// Paints one visual line, appending its runs to `runs`, and returns the index
/// of the first segment of the next line.
fn paint_line(
    segments: &[Segment],
    line_start: usize,
    available_width: Option<TerminalWidth>,
    runs: &mut Vec<Run>,
) -> usize {
    // Runs since the last fill; claimed into `chunks` once the next fill is hit.
    let mut pending: Vec<Run> = Vec::new();
    let mut chunks: Vec<(Vec<Run>, &FillSegment)> = Vec::new();
    let mut used_width = 0;
    // A fill resets the inheritance chain, so nothing after it can see across.
    let mut previous_style: Option<ResolvedStyle> = None;

    let mut next_segment = line_start;
    while let Some(segment) = segments.get(next_segment) {
        next_segment += 1;

        match segment {
            Segment::Fill(fill) => {
                chunks.push((std::mem::take(&mut pending), fill));
                previous_style = None;
            }
            Segment::Text(text) => {
                used_width += segment.width_graphemes();
                let style = ResolvedStyle::resolve(text.symbolic_style(), previous_style);
                previous_style = Some(style);
                pending.push(Run {
                    text: segment.value().to_owned(),
                    style,
                    kind: RunKind::Text,
                });
            }
            Segment::LineTerm => {
                // Nothing follows a line terminator, so inheritance simply ends here.
                pending.push(Run {
                    text: segment.value().to_owned(),
                    style: ResolvedStyle::plain(),
                    kind: RunKind::LineTerminator,
                });
                break;
            }
        }
    }

    if !chunks.is_empty() {
        let fill_count = chunks.len();

        for (index, (chunk, fill)) in chunks.into_iter().enumerate() {
            let fill_width = available_width.and_then(|TerminalWidth(available)| {
                (available > used_width).then(|| {
                    let leftover = available - used_width;
                    leftover / fill_count + usize::from(index < leftover % fill_count)
                })
            });
            let preceding_style = chunk.last().map(Run::style);
            let style = ResolvedStyle::resolve(fill.symbolic_style(), preceding_style);
            let text = fill.expand(fill_width);
            runs.extend(chunk);
            runs.push(Run {
                text,
                style,
                kind: RunKind::Fill,
            });
        }
    }

    runs.append(&mut pending);
    next_segment
}

/// The escape sequences needed to get from one resolved style to the next.
///
/// This mirrors the collapsing that `nu_ansi_term::AnsiStrings` performs, which
/// is not reachable as a library function.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StyleTransition {
    /// The next style only adds attributes; emit just those.
    AddAttributes(AnsiStyle),
    /// Attributes have to be switched off, which is only possible by resetting
    /// everything and starting the next style from scratch.
    ResetFirst,
    /// The styles are identical; emit nothing.
    Nothing,
}

impl StyleTransition {
    fn between(first: AnsiStyle, next: AnsiStyle) -> Self {
        if first == next {
            return Self::Nothing;
        }

        // None of these attributes can be switched off on its own.
        let switched_off = (first.is_bold && !next.is_bold)
            || (first.is_dimmed && !next.is_dimmed)
            || (first.is_italic && !next.is_italic)
            || (first.is_underline && !next.is_underline)
            || (first.is_blink && !next.is_blink)
            || (first.is_reverse && !next.is_reverse)
            || (first.is_hidden && !next.is_hidden)
            || (first.is_strikethrough && !next.is_strikethrough)
            || (first.foreground.is_some() && next.foreground.is_none())
            || (first.background.is_some() && next.background.is_none());

        if switched_off {
            return Self::ResetFirst;
        }

        let mut added = AnsiStyle::new();
        added.is_bold = first.is_bold != next.is_bold;
        added.is_dimmed = first.is_dimmed != next.is_dimmed;
        added.is_italic = first.is_italic != next.is_italic;
        added.is_underline = first.is_underline != next.is_underline;
        added.is_blink = first.is_blink != next.is_blink;
        added.is_reverse = first.is_reverse != next.is_reverse;
        added.is_hidden = first.is_hidden != next.is_hidden;
        added.is_strikethrough = first.is_strikethrough != next.is_strikethrough;
        if first.foreground != next.foreground {
            added.foreground = next.foreground;
        }
        if first.background != next.background {
            added.background = next.background;
        }

        Self::AddAttributes(added)
    }
}

/// Escapes the characters that would otherwise be read as markup delimiters.
fn escape_markup_text(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        if matches!(character, '\\' | '[' | ']') {
            escaped.push('\\');
        }
        escaped.push(character);
    }
    escaped
}

/// Spells a resolved style the way a starship format string would.
fn style_markup(style: AnsiStyle) -> String {
    let mut tokens: Vec<String> = Vec::new();

    if style.prefix_with_reset {
        // Not expressible in a format string; the configuration parser never
        // produces it, but the markup should not silently drop it either.
        tokens.push(String::from("reset"));
    }
    if let Some(color) = style.foreground {
        tokens.push(color_markup(color));
    }
    if let Some(color) = style.background {
        tokens.push(format!("bg:{}", color_markup(color)));
    }
    for (is_set, token) in [
        (style.is_bold, "bold"),
        (style.is_dimmed, "dimmed"),
        (style.is_italic, "italic"),
        (style.is_underline, "underline"),
        (style.is_blink, "blink"),
        (style.is_reverse, "inverted"),
        (style.is_hidden, "hidden"),
        (style.is_strikethrough, "strikethrough"),
    ] {
        if is_set {
            tokens.push(String::from(token));
        }
    }

    tokens.join(" ")
}

/// Spells a colour the way a starship format string would.
fn color_markup(color: Color) -> String {
    let named = match color {
        Color::Black => "black",
        Color::DarkGray => "bright-black",
        Color::Red => "red",
        Color::LightRed => "bright-red",
        Color::Green => "green",
        Color::LightGreen => "bright-green",
        Color::Yellow => "yellow",
        Color::LightYellow => "bright-yellow",
        Color::Blue => "blue",
        Color::LightBlue => "bright-blue",
        Color::Purple => "purple",
        Color::LightPurple => "bright-purple",
        Color::Cyan => "cyan",
        Color::LightCyan => "bright-cyan",
        Color::White => "white",
        Color::LightGray => "bright-white",
        Color::Magenta => "magenta",
        Color::LightMagenta => "bright-magenta",
        Color::Default => "default",
        // Fixed and Rgb have no named spelling; write them out directly instead.
        Color::Fixed(number) => return number.to_string(),
        Color::Rgb(red, green, blue) => return format!("#{red:02X}{green:02X}{blue:02X}"),
    };
    String::from(named)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::parse_style_string;
    use crate::formatter::StringFormatter;
    use nu_ansi_term::{AnsiString, AnsiStrings};

    /// Parses a style string the way a module's configuration would.
    fn style(style_string: &str) -> Option<Style> {
        if style_string.is_empty() {
            return None;
        }
        Some(parse_style_string(style_string, None).expect("style string should parse"))
    }

    /// One text segment. The value must not contain a newline: newlines are
    /// structural and are written as [`Segment::LineTerm`] instead.
    fn text(style_string: &str, value: &str) -> Segment {
        let mut segments = Segment::from_text(style(style_string), value);
        assert_eq!(1, segments.len(), "text segments cannot span lines");
        segments.remove(0)
    }

    /// One fill segment.
    fn fill(style_string: &str, value: &str) -> Segment {
        Segment::fill(style(style_string), value)
    }

    /// Parses a starship format string into the segments a module would emit.
    fn segments(format: &str) -> Vec<Segment> {
        StringFormatter::new(format)
            .expect("format string should parse")
            .parse(None, None)
            .expect("format string should evaluate")
    }

    /// The concrete style of every run, in order.
    fn styles(painted: &Painted) -> Vec<AnsiStyle> {
        painted
            .runs()
            .iter()
            .map(|run| run.style().as_ansi_style())
            .collect()
    }

    /// The bytes `nu_ansi_term` produces for the same runs. This is exactly what
    /// starship emitted before painting was made an explicit pass, so it is the
    /// reference that the output has to stay identical to.
    fn reference_bytes(painted: &Painted) -> String {
        let ansi_strings: Vec<AnsiString> = painted
            .runs()
            .iter()
            .map(|run| run.style().as_ansi_style().paint(run.text().to_owned()))
            .collect();
        AnsiStrings(&ansi_strings).to_string()
    }

    fn assert_matches_reference(painted: &Painted) {
        assert_eq!(reference_bytes(painted), painted.to_string());
    }

    #[test]
    fn empty_input_paints_nothing() {
        let painted = Painted::paint(&[], None);

        assert!(painted.is_empty());
        assert_eq!(0, painted.line_count());
        assert_eq!(String::new(), painted.to_string());
        assert_eq!(String::new(), painted.to_markup());
        assert_eq!(Vec::<u8>::new(), painted.to_bytes());
    }

    #[test]
    fn plain_text_is_emitted_without_escape_sequences() {
        let painted = Painted::paint(&[text("", "❯ ")], None);

        assert_eq!("❯ ", painted.to_string());
        assert_eq!("❯ ", painted.to_markup());
        assert_matches_reference(&painted);
    }

    #[test]
    fn previous_foreground_and_background_are_resolved_left_to_right() {
        let painted = Painted::paint(
            &segments("[a](fg:red bg:blue)[b](fg:prev_bg bg:prev_fg)"),
            None,
        );

        assert_eq!(
            vec![
                AnsiStyle::new().fg(Color::Red).on(Color::Blue),
                AnsiStyle::new().fg(Color::Blue).on(Color::Red),
            ],
            styles(&painted)
        );
        assert_eq!("[a](red bg:blue)[b](blue bg:red)", painted.to_markup());
        assert_matches_reference(&painted);
    }

    #[test]
    fn previous_colors_chain_across_many_segments() {
        let painted = Painted::paint(
            &segments("[a](fg:red)[b](fg:prev_fg)[c](fg:prev_fg)[d](bg:prev_fg)"),
            None,
        );

        assert_eq!(
            vec![
                AnsiStyle::new().fg(Color::Red),
                AnsiStyle::new().fg(Color::Red),
                AnsiStyle::new().fg(Color::Red),
                AnsiStyle::new().on(Color::Red),
            ],
            styles(&painted)
        );
        assert_matches_reference(&painted);
    }

    #[test]
    fn previous_colors_see_through_empty_segments() {
        let painted = Painted::paint(&segments("[](bg:#9A348E)[X](bg:prev_bg)"), None);

        assert_eq!(
            Some(Color::Rgb(0x9A, 0x34, 0x8E)),
            painted.runs()[1].style().as_ansi_style().background
        );
        assert_eq!("[](bg:#9A348E)[X](bg:#9A348E)", painted.to_markup());
        assert_matches_reference(&painted);
    }

    #[test]
    fn previous_colors_do_not_reach_across_a_line_terminator() {
        let prompt = vec![
            text("fg:red", "a"),
            Segment::LineTerm,
            text("fg:prev_fg", "b"),
        ];

        let painted = Painted::paint(&prompt, None);

        assert_eq!(2, painted.line_count());
        // With no left neighbour on its own line, the reference resolves to no
        // colour at all.
        assert_eq!(None, painted.runs()[2].style().as_ansi_style().foreground);
        assert_matches_reference(&painted);
    }

    #[test]
    fn lines_partition_the_runs() {
        let prompt = vec![
            text("red", "a"),
            Segment::LineTerm,
            text("green", "b"),
            Segment::LineTerm,
            text("", "c"),
        ];

        let painted = Painted::paint(&prompt, None);

        assert_eq!(3, painted.line_count());
        assert_eq!(Some(0..2), painted.line_range(LineIndex(0)));
        assert_eq!(Some(2..4), painted.line_range(LineIndex(1)));
        assert_eq!(Some(4..5), painted.line_range(LineIndex(2)));
        assert_eq!(None, painted.line_range(LineIndex(3)));

        assert_eq!(
            painted.line(LineIndex(1)),
            Some(&painted.runs()[2..4]),
            "line() and line_range() must agree"
        );

        let line_texts: Vec<String> = painted
            .lines()
            .map(|line| line.iter().map(Run::text).collect())
            .collect();
        assert_eq!(vec!["a\n", "b\n", "c"], line_texts);
    }

    #[test]
    fn line_terminators_are_structural_not_text() {
        let prompt = vec![text("", "a"), Segment::LineTerm, text("", "b")];

        let painted = Painted::paint(&prompt, None);

        assert_eq!(
            vec![RunKind::Text, RunKind::LineTerminator, RunKind::Text],
            painted.runs().iter().map(Run::kind).collect::<Vec<_>>()
        );
        for run in painted.runs() {
            if run.kind() == RunKind::Text {
                assert!(!run.text().contains('\n'));
            }
        }
        assert_eq!("a\nb", painted.to_string());
        assert_eq!("a\nb", painted.to_markup());
    }

    #[test]
    fn a_trailing_line_terminator_does_not_open_a_new_line() {
        let painted = Painted::paint(&[text("", "a"), Segment::LineTerm], None);

        assert_eq!(1, painted.line_count());
        assert_eq!(Some(0..2), painted.line_range(LineIndex(0)));
    }

    #[test]
    fn a_trailing_empty_text_segment_after_a_line_terminator_does_not_open_a_new_line() {
        let painted = Painted::paint(&Segment::from_text(None, "a\n"), None);

        assert_eq!(1, painted.line_count());
        assert_eq!("a\n", painted.to_string());
    }

    #[test]
    fn a_fill_absorbs_the_leftover_width() {
        let prompt = vec![text("red", "a"), fill("", "."), text("green", "b")];

        let painted = Painted::paint(&prompt, Some(TerminalWidth(10)));

        assert_eq!("........", painted.runs()[1].text());
        assert_matches_reference(&painted);
    }

    #[test]
    fn several_fills_share_the_leftover_width() {
        let prompt = vec![
            text("", "a"),
            fill("", "."),
            text("", "b"),
            fill("", "."),
            text("", "c"),
        ];

        let painted = Painted::paint(&prompt, Some(TerminalWidth(11)));

        assert_eq!("a....b....c", painted.to_string());
    }

    #[test]
    fn a_fill_keeps_its_natural_width_when_the_line_is_already_full() {
        let prompt = vec![text("", "abcdef"), fill("", "-:-")];

        let painted = Painted::paint(&prompt, Some(TerminalWidth(3)));

        assert_eq!("abcdef-:-", painted.to_string());
    }

    #[test]
    fn a_fill_keeps_its_natural_width_when_no_width_is_available() {
        let prompt = vec![text("", "a"), fill("", "-:-")];

        let painted = Painted::paint(&prompt, None);

        assert_eq!("a-:-", painted.to_string());
    }

    #[test]
    fn a_fill_inherits_from_the_run_to_its_left() {
        let prompt = vec![text("fg:red bg:blue", "a"), fill("fg:prev_bg", ".")];

        let painted = Painted::paint(&prompt, Some(TerminalWidth(5)));

        assert_eq!(
            Some(Color::Blue),
            painted.runs()[1].style().as_ansi_style().foreground
        );
        assert_matches_reference(&painted);
    }

    #[test]
    fn a_fill_breaks_the_inheritance_chain() {
        let prompt = vec![text("fg:red", "a"), fill("", "."), text("fg:prev_fg", "b")];

        let painted = Painted::paint(&prompt, Some(TerminalWidth(6)));

        // The run after the fill has no left neighbour it may inherit from.
        assert_eq!(None, painted.runs()[2].style().as_ansi_style().foreground);
        assert_matches_reference(&painted);
    }

    #[test]
    fn fills_are_measured_per_line() {
        let prompt = vec![
            text("", "a"),
            fill("", "."),
            Segment::LineTerm,
            text("", "bb"),
            fill("", "."),
        ];

        let painted = Painted::paint(&prompt, Some(TerminalWidth(6)));

        assert_eq!(2, painted.line_count());
        assert_eq!("a.....\nbb....", painted.to_string());
    }

    #[test]
    fn a_line_terminator_after_a_fill_still_ends_the_line() {
        let prompt = vec![
            text("", "a"),
            fill("", "."),
            text("", "b"),
            Segment::LineTerm,
            text("", "c"),
        ];

        let painted = Painted::paint(&prompt, Some(TerminalWidth(6)));

        assert_eq!(2, painted.line_count());
        assert_eq!("a....b\nc", painted.to_string());
    }

    #[test]
    fn wide_and_emoji_graphemes_are_measured_by_display_width() {
        let prompt = vec![text("", "👩‍👩‍👦‍👦"), fill("", "🟦")];

        let painted = Painted::paint(&prompt, Some(TerminalWidth(10)));

        // The family emoji occupies two cells, leaving eight for four blocks.
        assert_eq!("🟦🟦🟦🟦", painted.runs()[1].text());
        assert_eq!("👩‍👩‍👦‍👦🟦🟦🟦🟦", painted.to_string());
    }

    #[test]
    fn a_fill_never_overshoots_a_partially_filled_cell() {
        let prompt = vec![text("", "a"), fill("", "🟢🔵🟡")];

        let painted = Painted::paint(&prompt, Some(TerminalWidth(6)));

        // Five cells remain, so the third emoji would not fit.
        assert_eq!("🟢🔵", painted.runs()[1].text());
    }

    #[test]
    fn wide_graphemes_survive_serialising_unchanged() {
        let prompt = vec![text("red", "👩‍👩‍👦‍👦"), text("", "Ü")];

        let painted = Painted::paint(&prompt, None);

        assert_eq!("\u{1b}[31m👩‍👩‍👦‍👦\u{1b}[0mÜ", painted.to_string());
        assert_matches_reference(&painted);
    }

    #[test]
    fn adjacent_identical_styles_are_not_repeated() {
        let painted = Painted::paint(&[text("red", "a"), text("red", "b")], None);

        assert_eq!("\u{1b}[31mab\u{1b}[0m", painted.to_string());
        assert_matches_reference(&painted);
    }

    #[test]
    fn added_attributes_are_emitted_without_a_reset() {
        let painted = Painted::paint(&[text("green", "a"), text("green bold", "b")], None);

        assert_eq!("\u{1b}[32ma\u{1b}[1mb\u{1b}[0m", painted.to_string());
        assert_matches_reference(&painted);
    }

    #[test]
    fn removed_attributes_force_a_reset() {
        let painted = Painted::paint(&[text("green bold", "a"), text("green", "b")], None);

        assert_eq!(
            "\u{1b}[1;32ma\u{1b}[0m\u{1b}[32mb\u{1b}[0m",
            painted.to_string()
        );
        assert_matches_reference(&painted);
    }

    #[test]
    fn escape_sequence_collapsing_matches_nu_ansi_term_for_every_pair_of_styles() {
        let attribute_setters: [fn(&AnsiStyle) -> AnsiStyle; 8] = [
            AnsiStyle::bold,
            AnsiStyle::dimmed,
            AnsiStyle::italic,
            AnsiStyle::underline,
            AnsiStyle::blink,
            AnsiStyle::reverse,
            AnsiStyle::hidden,
            AnsiStyle::strikethrough,
        ];
        let colors = [
            None,
            Some(Color::Red),
            Some(Color::Fixed(120)),
            Some(Color::Rgb(1, 2, 3)),
        ];

        let mut attribute_sets: Vec<AnsiStyle> = vec![AnsiStyle::new()];
        for setter in attribute_setters {
            attribute_sets.push(setter(&AnsiStyle::new()));
            attribute_sets.push(setter(&AnsiStyle::new().bold()));
        }
        let mut candidates: Vec<AnsiStyle> = Vec::new();
        for attributes in attribute_sets {
            for foreground in colors {
                for background in colors {
                    let mut candidate = attributes;
                    candidate.foreground = foreground;
                    candidate.background = background;
                    candidates.push(candidate);
                }
            }
        }

        for &first in &candidates {
            for &second in &candidates {
                let painted = Painted::paint(
                    &[
                        text_with_ansi_style(first, "first"),
                        text_with_ansi_style(second, "second"),
                    ],
                    None,
                );
                assert_eq!(
                    reference_bytes(&painted),
                    painted.to_string(),
                    "collapsing {first:?} into {second:?}"
                );
            }
        }
    }

    /// A text segment carrying a concrete style. Concrete styles hold no
    /// symbolic reference, so painting reproduces them exactly.
    fn text_with_ansi_style(style: AnsiStyle, value: &str) -> Segment {
        let mut segments = Segment::from_text(Some(Style::from(style)), value);
        assert_eq!(1, segments.len(), "text segments cannot span lines");
        segments.remove(0)
    }

    #[test]
    fn markup_spells_out_colors_and_attributes() {
        let painted = Painted::paint(
            &segments("via [ v12.0.0 ](green bold)[x](fg:120 bg:#0A0B0C italic underline)"),
            None,
        );

        assert_eq!(
            "via [ v12.0.0 ](green bold)[x](120 bg:#0A0B0C italic underline)",
            painted.to_markup()
        );
        assert_eq!(painted.to_markup(), format!("{painted:?}"));
    }

    #[test]
    fn markup_shows_fills_and_line_terminators() {
        let prompt = vec![
            text("red", "a"),
            fill("blue", "."),
            Segment::LineTerm,
            text("", "b"),
        ];

        let painted = Painted::paint(&prompt, Some(TerminalWidth(5)));

        assert_eq!("[a](red)[....](blue)\nb", painted.to_markup());
    }

    #[test]
    fn markup_escapes_its_own_delimiters() {
        let painted = Painted::paint(&[text("red", "[a]"), text("", "back\\slash")], None);

        assert_eq!("[\\[a\\]](red)back\\\\slash", painted.to_markup());
    }

    #[test]
    fn markup_names_every_attribute() {
        let painted = Painted::paint(
            &[text(
                "bold dimmed italic underline blink inverted hidden strikethrough",
                "a",
            )],
            None,
        );

        assert_eq!(
            "[a](bold dimmed italic underline blink inverted hidden strikethrough)",
            painted.to_markup()
        );
    }

    #[test]
    fn to_bytes_agrees_with_the_display_form() {
        let prompt = vec![text("red bold", "a"), Segment::LineTerm, text("blue", "b")];

        let painted = Painted::paint(&prompt, None);

        assert_eq!(painted.to_string().into_bytes(), painted.to_bytes());
    }
}
