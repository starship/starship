//! Terminal-emulator support for repaint tests.

use alacritty_terminal::event::{Event, EventListener};
use alacritty_terminal::grid::Dimensions;
use alacritty_terminal::index::{Line, Point};
use alacritty_terminal::term::cell::Flags;
use alacritty_terminal::term::{Config, Term};
use alacritty_terminal::vte::ansi::{Color as TerminalColor, NamedColor, Processor};

use crate::module::painted::{Painted, TerminalWidth};

pub const SCREEN_ROWS: usize = 16;

#[derive(Clone)]
struct DiscardEvents;

impl EventListener for DiscardEvents {
    fn send_event(&self, _: Event) {}
}

struct ScreenSize {
    columns: usize,
}

impl Dimensions for ScreenSize {
    fn total_lines(&self) -> usize {
        SCREEN_ROWS
    }

    fn screen_lines(&self) -> usize {
        SCREEN_ROWS
    }

    fn columns(&self) -> usize {
        self.columns
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CellSnapshot {
    character: char,
    foreground: TerminalColor,
    background: TerminalColor,
    flags: Flags,
}

impl CellSnapshot {
    fn is_blank(&self) -> bool {
        *self == Self::blank()
    }

    fn blank() -> Self {
        Self {
            character: ' ',
            foreground: TerminalColor::Named(NamedColor::Foreground),
            background: TerminalColor::Named(NamedColor::Background),
            flags: Flags::empty(),
        }
    }
}

impl std::fmt::Debug for CellSnapshot {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}", self.character)?;
        if self.foreground != Self::blank().foreground {
            write!(formatter, "/fg:{:?}", self.foreground)?;
        }
        if self.background != Self::blank().background {
            write!(formatter, "/bg:{:?}", self.background)?;
        }
        if !self.flags.is_empty() {
            write!(formatter, "/{:?}", self.flags)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq)]
pub struct Screen {
    rows: Vec<Vec<CellSnapshot>>,
    cursor: Point,
}

impl Screen {
    fn of(terminal: &Term<DiscardEvents>) -> Self {
        let grid = terminal.grid();
        let mut rows = Vec::with_capacity(grid.screen_lines());

        for row in 0..grid.screen_lines() {
            let mut cells: Vec<CellSnapshot> = grid[Line(row as i32)]
                .into_iter()
                .map(|cell| CellSnapshot {
                    character: cell.c,
                    foreground: cell.fg,
                    background: cell.bg,
                    // `WRAPLINE` is transport history, not screen state.
                    flags: cell.flags.difference(Flags::WRAPLINE),
                })
                .collect();
            while cells.last().is_some_and(CellSnapshot::is_blank) {
                cells.pop();
            }
            rows.push(cells);
        }

        while rows.last().is_some_and(Vec::is_empty) {
            rows.pop();
        }

        Self {
            rows,
            cursor: grid.cursor.point,
        }
    }
}

impl std::fmt::Debug for Screen {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(formatter, "cursor at {:?}", self.cursor)?;
        for (index, row) in self.rows.iter().enumerate() {
            let text: String = row.iter().map(|cell| cell.character).collect();
            writeln!(formatter, "  row {index}: {text:?} {row:?}")?;
        }
        Ok(())
    }
}

pub struct EmulatedTerminal {
    terminal: Term<DiscardEvents>,
    parser: Processor,
    width: TerminalWidth,
}

impl EmulatedTerminal {
    pub fn blank(width: TerminalWidth) -> Self {
        Self {
            terminal: Term::new(
                Config::default(),
                &ScreenSize { columns: width.0 },
                DiscardEvents,
            ),
            parser: Processor::new(),
            width,
        }
    }

    /// Applies the tty's `ONLCR` translation (`\n` becomes `\r\n`) before feeding.
    pub fn feed(&mut self, bytes: &[u8]) {
        let mut translated = Vec::with_capacity(bytes.len());
        for &byte in bytes {
            if byte == b'\n' {
                translated.push(b'\r');
            }
            translated.push(byte);
        }
        self.parser.advance(&mut self.terminal, &translated);
    }

    pub fn redraw(&mut self, bytes: &[u8]) {
        *self = Self::blank(self.width);
        self.feed(bytes);
    }

    pub fn screen(&self) -> Screen {
        Screen::of(&self.terminal)
    }
}

pub fn fully_rendered(painted: &Painted, width: TerminalWidth) -> Screen {
    let mut terminal = EmulatedTerminal::blank(width);
    terminal.feed(&painted.to_bytes());
    terminal.screen()
}
