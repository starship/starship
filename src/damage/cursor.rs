//! Cursor-neutral terminal writes.

use std::fmt;
use std::io::Write;

use nu_ansi_term::ansi::RESET;

use super::Column;
use crate::module::painted::ResolvedStyle;

const SAVE_CURSOR: &[u8] = b"\x1b7";
const RESTORE_CURSOR: &[u8] = b"\x1b8";
const ERASE_TO_END_OF_LINE: &[u8] = b"\x1b[K";

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RowsAbove(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CursorSafeText<'a>(&'a str);

impl<'a> CursorSafeText<'a> {
    pub fn new(text: &'a str) -> Option<Self> {
        text.chars()
            .all(|character| !character.is_control())
            .then_some(Self(text))
    }
}

pub struct CursorNeutralBody {
    bytes: Vec<u8>,
    style: Option<ResolvedStyle>,
}

impl CursorNeutralBody {
    fn new() -> Self {
        Self {
            bytes: Vec::new(),
            style: None,
        }
    }

    pub fn move_to(&mut self, rows: RowsAbove, column: Column) {
        if rows.0 != 0 {
            write!(self.bytes, "\x1b[{}A", rows.0).expect("a vector is writable");
        }
        write!(self.bytes, "\x1b[{}G", column.0 + 1).expect("a vector is writable");
    }

    pub fn select_style(&mut self, style: ResolvedStyle) {
        if self.style == Some(style) {
            return;
        }
        self.bytes.extend_from_slice(RESET.as_bytes());
        write!(self.bytes, "{}", style.as_ansi_style().prefix()).expect("a vector is writable");
        self.style = Some(style);
    }

    pub fn write_text(&mut self, text: CursorSafeText<'_>) {
        self.bytes.extend_from_slice(text.0.as_bytes());
    }

    pub fn erase_to_end_of_line(&mut self) {
        self.select_style(ResolvedStyle::plain());
        self.bytes.extend_from_slice(ERASE_TO_END_OF_LINE);
    }
}

/// A terminal write that restores the cursor.
#[derive(Clone, PartialEq, Eq)]
pub struct CursorNeutral(Vec<u8>);

impl CursorNeutral {
    pub fn around(build: impl FnOnce(&mut CursorNeutralBody)) -> Self {
        let mut body = CursorNeutralBody::new();
        build(&mut body);

        let mut bytes =
            Vec::with_capacity(SAVE_CURSOR.len() + body.bytes.len() + RESTORE_CURSOR.len());
        bytes.extend_from_slice(SAVE_CURSOR);
        bytes.extend_from_slice(&body.bytes);
        bytes.extend_from_slice(RESTORE_CURSOR);
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for CursorNeutral {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "CursorNeutral({:?})",
            String::from_utf8_lossy(&self.0)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_cannot_emit_a_control_character_as_text() {
        assert!(CursorSafeText::new("ok\n").is_none());
    }

    #[test]
    fn every_write_restores_the_saved_cursor() {
        let repaint = CursorNeutral::around(|body| {
            body.move_to(RowsAbove(1), Column(2));
            body.write_text(CursorSafeText::new("ok").unwrap());
        });
        assert!(repaint.as_bytes().starts_with(SAVE_CURSOR));
        assert!(repaint.as_bytes().ends_with(RESTORE_CURSOR));
    }
}
