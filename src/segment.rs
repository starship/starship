use crate::{
    config::Style,
    print::{Grapheme, UnicodeWidthGraphemes},
};
use nu_ansi_term::{AnsiString, Style as AnsiStyle};
use unicode_segmentation::UnicodeSegmentation;

/// Type that holds text with an associated style
#[derive(Clone)]
pub struct TextSegment {
    /// The segment's style. If None, will inherit the style of the module containing it.
    style: Option<Style>,

    /// The string value of the current segment.
    value: String,
}

impl TextSegment {
    // Returns the AnsiString of the segment value
    fn ansi_string(&self, prev: Option<&AnsiStyle>) -> AnsiString<'_> {
        match self.style {
            Some(style) => style.to_ansi_style(prev).paint(&self.value),
            None => AnsiString::from(&self.value),
        }
    }
}

/// Type that holds fill text with an associated style
#[derive(Clone)]
pub struct FillSegment {
    /// The segment's style. If None, will inherit the style of the module containing it.
    style: Option<Style>,

    /// The string value of the current segment.
    value: String,
}

impl FillSegment {
    // Returns the AnsiString of the segment value, not including its prefix and suffix
    pub fn ansi_string(&self, width: Option<usize>, prev: Option<&AnsiStyle>) -> AnsiString<'_> {
        let s = match width {
            Some(w) => self
                .value
                .graphemes(true)
                // Zero-width graphemes (e.g. a bare combining mark or ZWJ used as the fill
                // symbol) never advance `len` below, so cycling over them would spin
                // forever without ever reaching `w`. Drop them before cycling: they can't
                // contribute visible width anyway, and this guarantees termination even
                // if every grapheme in the symbol turns out to be zero-width.
                .filter(|g| Grapheme(g).width() > 0)
                .cycle()
                .scan(0usize, |len, g| {
                    *len += Grapheme(g).width();
                    if *len <= w { Some(g) } else { None }
                })
                .collect::<String>(),
            None => String::from(&self.value),
        };
        match self.style {
            Some(style) => style.to_ansi_style(prev).paint(s),
            None => AnsiString::from(s),
        }
    }
}

#[cfg(test)]
mod fill_seg_tests {
    use super::FillSegment;
    use nu_ansi_term::{AnsiString, Color};

    #[test]
    fn ansi_string_width() {
        let width: usize = 10;
        let style = Color::Blue.bold();

        let inputs = vec![
            (".", ".........."),
            (".:", ".:.:.:.:.:"),
            ("-:-", "-:--:--:--"),
            ("🟦", "🟦🟦🟦🟦🟦"),
            ("🟢🔵🟡", "🟢🔵🟡🟢🔵"),
        ];

        for (text, expected) in &inputs {
            let f = FillSegment {
                value: String::from(*text),
                style: Some(style.into()),
            };
            let actual = f.ansi_string(Some(width), None);
            assert_eq!(style.paint(*expected), actual);
        }
    }

    #[test]
    fn ansi_string_all_zero_width_symbol_does_not_hang() {
        // A fill symbol made up entirely of zero-width graphemes (e.g. a bare
        // combining mark or a ZWJ) can never reach the target width, so cycling
        // over it must terminate instead of looping forever. It can't render
        // anything visible, so the result should just be empty.
        let f = FillSegment {
            value: String::from("\u{0301}"), // combining acute accent, width 0
            style: None,
        };
        let actual = f.ansi_string(Some(10), None);
        assert_eq!(AnsiString::from(""), actual);
    }

    #[test]
    fn ansi_string_mixed_zero_width_symbol() {
        // Zero-width graphemes mixed into an otherwise-visible symbol should be
        // skipped, filling with just the visible graphemes instead of hanging.
        let width: usize = 4;
        let f = FillSegment {
            value: String::from("\u{200d}."), // ZWJ + '.'
            style: None,
        };
        let actual = f.ansi_string(Some(width), None);
        assert_eq!(AnsiString::from("...."), actual);
    }
}

/// A segment is a styled text chunk ready for printing.
#[derive(Clone)]
pub enum Segment {
    Text(TextSegment),
    Fill(FillSegment),
    LineTerm,
}

impl Segment {
    /// Creates new segments from a text with a style; breaking out `LineTerminators`.
    pub fn from_text<T>(style: Option<Style>, value: T) -> Vec<Self>
    where
        T: Into<String>,
    {
        let mut segs: Vec<Self> = Vec::new();
        value.into().split(LINE_TERMINATOR).for_each(|s| {
            if !segs.is_empty() {
                segs.push(Self::LineTerm);
            }
            segs.push(Self::Text(TextSegment {
                value: String::from(s),
                style,
            }));
        });
        segs
    }

    /// Creates a new fill segment
    pub fn fill<T>(style: Option<Style>, value: T) -> Self
    where
        T: Into<String>,
    {
        Self::Fill(FillSegment {
            style,
            value: value.into(),
        })
    }

    pub fn style(&self) -> Option<AnsiStyle> {
        match self {
            Self::Fill(fs) => fs.style.map(|cs| cs.to_ansi_style(None)),
            Self::Text(ts) => ts.style.map(|cs| cs.to_ansi_style(None)),
            Self::LineTerm => None,
        }
    }

    pub fn set_style_if_empty(&mut self, style: Option<Style>) {
        match self {
            Self::Fill(fs) => {
                if fs.style.is_none() {
                    fs.style = style;
                }
            }
            Self::Text(ts) => {
                if ts.style.is_none() {
                    ts.style = style;
                }
            }
            Self::LineTerm => {}
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Self::Fill(fs) => &fs.value,
            Self::Text(ts) => &ts.value,
            Self::LineTerm => LINE_TERMINATOR_STRING,
        }
    }

    // Returns the AnsiString of the segment value, not including its prefix and suffix
    pub fn ansi_string(&self, prev: Option<&AnsiStyle>) -> AnsiString<'_> {
        match self {
            Self::Fill(fs) => fs.ansi_string(None, prev),
            Self::Text(ts) => ts.ansi_string(prev),
            Self::LineTerm => AnsiString::from(LINE_TERMINATOR_STRING),
        }
    }

    pub fn width_graphemes(&self) -> usize {
        match self {
            Self::Fill(fs) => fs.value.width_graphemes(),
            Self::Text(ts) => ts.value.width_graphemes(),
            Self::LineTerm => 0,
        }
    }
}

const LINE_TERMINATOR: char = '\n';
const LINE_TERMINATOR_STRING: &str = "\n";
