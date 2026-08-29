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
        let previous_cursor = compute_cursor_row(previous);
        let next_cursor = compute_cursor_row(next);

        if previous.line_count() != next.line_count() || previous_cursor != next_cursor {
            return Self::Full;
        }

        let mut changed_line_data = None;

        for (line_index, (previous_runs, next_runs)) in
            previous.lines().zip(next.lines()).enumerate()
        {
            let previous_line = Line::new(previous_runs);
            let next_line = Line::new(next_runs);

            if previous_line.exceeds_width(terminal_width)
                || next_line.exceeds_width(terminal_width)
                || previous_line.has_zero_width_cell()
                || next_line.has_zero_width_cell()
            {
                return Self::Full;
            }

            if let Some(span) = Span::compare(previous_line, next_line) {
                // Only one changed line is repaintable incrementally.
                if changed_line_data.is_some() {
                    return Self::Full;
                }
                changed_line_data = Some((LineIndex(line_index), span, next_line));
            }
        }

        let Some((changed_index, span, next_line)) = changed_line_data else {
            return Self::None;
        };

        if changed_index.0 == next_cursor && span.erases_tail {
            return Self::Full;
        }

        let rows_above = RowsAbove(previous_cursor.saturating_sub(changed_index.0));
        span.paint(rows_above, next_line)
            .map_or(Self::Full, Self::Repaint)
    }
}

fn compute_cursor_row(painted: &Painted) -> usize {
    let total_lines = painted.line_count();
    let last_index = total_lines.saturating_sub(1);

    let has_trailing_terminator = painted
        .line(LineIndex(last_index))
        .and_then(|runs| runs.last())
        .is_some_and(|run| run.kind() == RunKind::LineTerminator);

    if has_trailing_terminator {
        total_lines
    } else {
        last_index
    }
}

#[derive(Clone, Copy)]
struct Line<'a> {
    runs: &'a [Run],
}

impl<'a> Line<'a> {
    fn new(runs: &'a [Run]) -> Self {
        Self { runs }
    }

    fn cells(self) -> impl Iterator<Item = Cell<'a>> {
        self.runs
            .iter()
            .filter(|run| run.kind() != RunKind::LineTerminator)
            .flat_map(|run| {
                let style = run.style();
                run.text()
                    .graphemes(true)
                    .map(move |text| Cell { text, style })
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
                    .map(move |text| Cell { text, style })
            })
    }

    fn cell_count(self) -> usize {
        self.cells().count()
    }

    fn width(self) -> TerminalWidth {
        let total_width = self.cells().map(|cell| Grapheme(cell.text).width()).sum();
        TerminalWidth(total_width)
    }

    fn exceeds_width(self, max_width: TerminalWidth) -> bool {
        self.width() > max_width
    }

    fn has_zero_width_cell(self) -> bool {
        self.cells().any(|cell| Grapheme(cell.text).width() == 0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cell<'a> {
    text: &'a str,
    style: ResolvedStyle,
}

#[derive(Clone, Copy)]
struct Span {
    start_index: usize,
    end_index: usize,
    start_column: Column,
    erases_tail: bool,
}

impl Span {
    fn compare(previous: Line<'_>, next: Line<'_>) -> Option<Self> {
        let (start_index, start_column) = Self::find_divergence(previous, next)?;

        let previous_count = previous.cell_count();
        let next_count = next.cell_count();

        // Suffix matching only makes sense when the lengths match.
        let suffix_length = if previous_count == next_count && previous.width() == next.width() {
            Self::match_suffix_length(previous, next, previous_count.saturating_sub(start_index))
        } else {
            usize::default()
        };

        Some(Self {
            start_index,
            end_index: next_count.saturating_sub(suffix_length),
            start_column,
            erases_tail: previous.width() != next.width(),
        })
    }

    // Single pass, tracking column alongside index.
    fn find_divergence(previous: Line<'_>, next: Line<'_>) -> Option<(usize, Column)> {
        let mut current_column = Column::default();
        let mut current_index = usize::default();

        let mut previous_cells = previous.cells();
        let mut next_cells = next.cells();

        loop {
            match (previous_cells.next(), next_cells.next()) {
                (Some(previous_cell), Some(next_cell)) if previous_cell == next_cell => {
                    current_column.0 += Grapheme(next_cell.text).width();
                    current_index += 1;
                }
                (Some(_), Some(_)) | (Some(_), None) | (None, Some(_)) => {
                    return Some((current_index, current_column));
                }
                (None, None) => return None,
            }
        }
    }

    fn match_suffix_length(previous: Line<'_>, next: Line<'_>, max_search_length: usize) -> usize {
        previous
            .cells_from_end()
            .zip(next.cells_from_end())
            .take(max_search_length)
            .take_while(|(previous_cell, next_cell)| previous_cell == next_cell)
            .count()
    }

    fn paint(self, rows_above: RowsAbove, next_line: Line<'_>) -> Option<CursorNeutral> {
        // A closure avoids allocating twice below.
        let cell_iterator = || {
            next_line
                .cells()
                .skip(self.start_index)
                .take(self.end_index.saturating_sub(self.start_index))
        };

        let is_cursor_safe = cell_iterator().all(|cell| CursorSafeText::new(cell.text).is_some());
        if !is_cursor_safe {
            return None;
        }

        Some(CursorNeutral::around(|terminal| {
            terminal.move_to(rows_above, self.start_column);

            for cell in cell_iterator() {
                terminal.select_style(cell.style);
                terminal.write_text(CursorSafeText::new(cell.text).unwrap()); // checked above
            }

            terminal.select_style(ResolvedStyle::plain());

            if self.erases_tail {
                terminal.erase_to_end_of_line();
            }
        }))
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
