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

impl Default for CellSnapshot {
    fn default() -> Self {
        Self {
            character: ' ',
            foreground: TerminalColor::Named(NamedColor::Foreground),
            background: TerminalColor::Named(NamedColor::Background),
            flags: Flags::empty(),
        }
    }
}

impl CellSnapshot {
    fn is_blank(&self) -> bool {
        *self == Self::default()
    }
}

impl std::fmt::Debug for CellSnapshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let default = Self::default();
        write!(f, "{:?}", self.character)?;
        if self.foreground != default.foreground {
            write!(f, "/fg:{:?}", self.foreground)?;
        }
        if self.background != default.background {
            write!(f, "/bg:{:?}", self.background)?;
        }
        if !self.flags.is_empty() {
            write!(f, "/{:?}", self.flags)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq)]
pub struct Screen {
    rows: Vec<Vec<CellSnapshot>>,
    cursor: Point,
}

impl From<&Term<DiscardEvents>> for Screen {
    fn from(terminal: &Term<DiscardEvents>) -> Self {
        let grid = terminal.grid();

        let mut rows: Vec<Vec<CellSnapshot>> = (0..grid.screen_lines())
            .map(|row| {
                let mut cells: Vec<_> = grid[Line(row as i32)]
                    .into_iter()
                    .map(|cell| CellSnapshot {
                        character: cell.c,
                        foreground: cell.fg,
                        background: cell.bg,
                        flags: cell.flags.difference(Flags::WRAPLINE),
                    })
                    .collect();

                cells.truncate(
                    cells
                        .iter()
                        .rposition(|c| !c.is_blank())
                        .map_or(0, |i| i + 1),
                );
                cells
            })
            .collect();

        // Same trick, for trailing empty rows.
        rows.truncate(
            rows.iter()
                .rposition(|r| !r.is_empty())
                .map_or(0, |i| i + 1),
        );

        Self {
            rows,
            cursor: grid.cursor.point,
        }
    }
}

impl std::fmt::Debug for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "cursor at {:?}", self.cursor)?;
        for (i, row) in self.rows.iter().enumerate() {
            let text: String = row.iter().map(|c| c.character).collect();
            writeln!(f, "  row {i}: {text:?} {row:?}")?;
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
        let translated: Vec<u8> = bytes
            .iter()
            .flat_map(|&b| {
                (b == b'\n')
                    .then_some(b'\r')
                    .into_iter()
                    .chain(std::iter::once(b))
            })
            .collect();

        self.parser.advance(&mut self.terminal, &translated);
    }

    pub fn redraw(&mut self, bytes: &[u8]) {
        *self = Self::blank(self.width);
        self.feed(bytes);
    }

    pub fn screen(&self) -> Screen {
        Screen::from(&self.terminal)
    }
}

pub fn fully_rendered(painted: &Painted, width: TerminalWidth) -> Screen {
    let mut terminal = EmulatedTerminal::blank(width);
    terminal.feed(&painted.to_bytes());
    terminal.screen()
}
