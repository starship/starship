//! Incremental, cursor-neutral prompt repainting.

pub mod cursor;
#[cfg(test)]
mod property_tests;
#[cfg(test)]
pub mod terminal;

use cursor::{CursorNeutral, CursorSafeText, RowsAbove};
use unicode_segmentation::UnicodeSegmentation;

use crate::module::painted::{LineIndex, Painted, ResolvedStyle, Run, RunKind, TerminalWidth};
use crate::print::Grapheme;

/// A zero-based display column.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Column(pub usize);

/// The result of comparing two painted prompts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Damage {
    None,
    Repaint(CursorNeutral),
    Full,
}

impl Damage {
    pub fn between(previous: &Painted, next: &Painted, terminal_width: TerminalWidth) -> Self {
        let previous_cursor = cursor_row(previous);
        let next_cursor = cursor_row(next);
        if previous.line_count() != next.line_count() || previous_cursor != next_cursor {
            return Self::Full;
        }

        let mut changed = None;
        for (index, (previous, next)) in previous.lines().zip(next.lines()).enumerate() {
            let previous = Line::new(previous);
            let next = Line::new(next);
            if previous.width() > terminal_width || next.width() > terminal_width {
                return Self::Full;
            }

            let Some(span) = Span::between(previous, next) else {
                continue;
            };
            if changed.replace((LineIndex(index), span, next)).is_some() {
                return Self::Full;
            }
        }

        let Some((line, span, next)) = changed else {
            return Self::None;
        };
        if line.0 == next_cursor && span.erases_tail {
            return Self::Full;
        }

        let rows = RowsAbove(previous_cursor - line.0);
        span.paint(rows, next).map_or(Self::Full, Self::Repaint)
    }
}

fn cursor_row(painted: &Painted) -> usize {
    let last = painted.line_count().saturating_sub(1);
    if painted
        .line(LineIndex(last))
        .and_then(<[_]>::last)
        .is_some_and(|run| run.kind() == RunKind::LineTerminator)
    {
        painted.line_count()
    } else {
        last
    }
}

#[derive(Clone, Copy)]
struct Line<'a> {
    runs: &'a [Run],
    width: TerminalWidth,
}

impl<'a> Line<'a> {
    fn new(runs: &'a [Run]) -> Self {
        let width = runs
            .iter()
            .filter(|run| run.kind() != RunKind::LineTerminator)
            .flat_map(|run| run.text().graphemes(true))
            .map(Grapheme)
            .map(|grapheme| grapheme.width())
            .sum();
        Self {
            runs,
            width: TerminalWidth(width),
        }
    }

    fn width(self) -> TerminalWidth {
        self.width
    }

    fn cells(self) -> impl Iterator<Item = Cell<'a>> {
        self.runs
            .iter()
            .filter(|run| run.kind() != RunKind::LineTerminator)
            .flat_map(|run| {
                let style = run.style();
                run.text().graphemes(true).map(move |text| (text, style))
            })
            .scan(Column::default(), |column, (text, style)| {
                let cell = Cell {
                    text,
                    style,
                    column: *column,
                };
                column.0 += Grapheme(text).width();
                Some(cell)
            })
    }

    fn cells_from_end(self) -> impl Iterator<Item = Cell<'a>> {
        self.runs
            .iter()
            .rev()
            .filter(|run| run.kind() != RunKind::LineTerminator)
            .flat_map(|run| {
                let style = run.style();
                run.text()
                    .graphemes(true)
                    .rev()
                    .map(move |text| (text, style))
            })
            .scan(self.width.0, |column, (text, style)| {
                *column -= Grapheme(text).width();
                Some(Cell {
                    text,
                    style,
                    column: Column(*column),
                })
            })
    }

    fn cell_count(self) -> usize {
        self.cells().count()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cell<'a> {
    text: &'a str,
    style: ResolvedStyle,
    column: Column,
}

#[derive(Clone, Copy)]
struct Span {
    first: usize,
    last: usize,
    column: Column,
    erases_tail: bool,
}

impl Span {
    fn between(previous: Line<'_>, next: Line<'_>) -> Option<Self> {
        let first = previous
            .cells()
            .zip(next.cells())
            .take_while(|(previous, next)| previous == next)
            .count();
        let previous_count = previous.cell_count();
        let next_count = next.cell_count();
        if first == previous_count && first == next_count {
            return None;
        }

        let suffix = if previous_count == next_count {
            previous
                .cells_from_end()
                .zip(next.cells_from_end())
                .take(previous_count - first)
                .take_while(|(previous, next)| previous == next)
                .count()
        } else {
            0
        };

        Some(Self {
            first,
            last: next_count - suffix,
            column: next
                .cells()
                .nth(first)
                .map_or(Column(next.width().0), |cell| cell.column),
            erases_tail: previous.width() != next.width(),
        })
    }

    fn paint(self, rows: RowsAbove, line: Line<'_>) -> Option<CursorNeutral> {
        let span = || line.cells().skip(self.first).take(self.last - self.first);
        span()
            .all(|cell| CursorSafeText::new(cell.text).is_some())
            .then(|| {
                CursorNeutral::around(|body| {
                    body.move_to(rows, self.column);
                    for cell in span() {
                        body.select_style(cell.style);
                        body.write_text(CursorSafeText::new(cell.text).unwrap());
                    }
                    body.select_style(ResolvedStyle::plain());
                    self.erases_tail.then(|| body.erase_to_end_of_line());
                })
            })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::config::{Style, parse_style_string};
    use crate::formatter::StringFormatter;
    use crate::segment::Segment;

    fn parsed_style(value: &str) -> Option<Style> {
        (!value.is_empty()).then(|| parse_style_string(value, None).unwrap())
    }

    pub fn text(style: &str, value: &str) -> Segment {
        Segment::from_text(parsed_style(style), value).remove(0)
    }

    pub fn segments(format: &str) -> Vec<Segment> {
        StringFormatter::new(format)
            .unwrap()
            .parse(None, None)
            .unwrap()
    }

    pub fn painted(segments: &[Segment], width: usize) -> Painted {
        Painted::paint(segments, Some(TerminalWidth(width)))
    }

    fn with_cursor_line(segments: &[Segment]) -> Vec<Segment> {
        let mut prompt = segments.to_vec();
        prompt.extend([Segment::LineTerm, text("", "> ")]);
        prompt
    }

    fn assert_replays(previous: Painted, next: Painted, width: TerminalWidth) {
        let mut terminal = crate::damage::terminal::EmulatedTerminal::blank(width);
        terminal.feed(&previous.to_bytes());
        match Damage::between(&previous, &next, width) {
            Damage::None => {}
            Damage::Repaint(bytes) => terminal.feed(bytes.as_bytes()),
            Damage::Full => terminal.redraw(&next.to_bytes()),
        }
        assert_eq!(
            crate::damage::terminal::fully_rendered(&next, width),
            terminal.screen()
        );
    }

    #[test]
    fn unchanged_paint_is_empty_damage() {
        let prompt = painted(&with_cursor_line(&segments("[main](red)")), 40);
        assert_eq!(
            Damage::None,
            Damage::between(&prompt, &prompt, TerminalWidth(40))
        );
    }

    #[test]
    fn repaint_touches_one_changed_run() {
        let previous = painted(&with_cursor_line(&segments("[main](red)")), 40);
        let next = painted(&with_cursor_line(&segments("[work](red)")), 40);
        let Damage::Repaint(bytes) = Damage::between(&previous, &next, TerminalWidth(40)) else {
            panic!("same-shape edit should repaint");
        };
        assert!(String::from_utf8_lossy(bytes.as_bytes()).contains("work"));
        assert_replays(previous, next, TerminalWidth(40));
    }

    #[test]
    fn a_fill_resynchronizes_after_a_width_change() {
        let previous = painted(&with_cursor_line(&segments("a$fill>b")), 20);
        let next = painted(&with_cursor_line(&segments("long$fill>b")), 20);
        assert_replays(previous, next, TerminalWidth(20));
    }

    #[test]
    fn a_width_change_on_the_cursor_row_requires_a_redraw() {
        let previous = painted(&segments("main >"), 40);
        let next = painted(&segments("workspace >"), 40);
        assert_eq!(
            Damage::Full,
            Damage::between(&previous, &next, TerminalWidth(40))
        );
    }

    #[test]
    fn two_changed_lines_require_a_redraw() {
        let previous = painted(&segments("one\ntwo\n> "), 40);
        let next = painted(&segments("uno\ndos\n> "), 40);
        assert_eq!(
            Damage::Full,
            Damage::between(&previous, &next, TerminalWidth(40))
        );
    }

    #[test]
    fn repaint_reproduces_styled_cells() {
        let previous = painted(
            &with_cursor_line(&segments("[one](red)[ two](fg:prev_fg)")),
            40,
        );
        let next = painted(
            &with_cursor_line(&segments("[one](green)[ two](fg:prev_fg)")),
            40,
        );
        assert_replays(previous, next, TerminalWidth(40));
    }
}
