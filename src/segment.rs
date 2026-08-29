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
    pub(crate) fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// The segment's style before any `prev_fg`/`prev_bg` reference in it has
    /// been resolved.
    pub(crate) fn symbolic_style(&self) -> Option<Style> {
        self.style
    }

    fn ansi_string(&self, previous: Option<&AnsiStyle>) -> AnsiString<'_> {
        self.style.map_or_else(
            || AnsiString::from(&self.value),
            |style| style.to_ansi_style(previous).paint(&self.value),
        )
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
    /// The segment's style before any `prev_fg`/`prev_bg` reference in it has
    /// been resolved.
    pub(crate) fn symbolic_style(&self) -> Option<Style> {
        self.style
    }

    /// Repeats the fill's value until it occupies `width` terminal cells,
    /// stopping before any grapheme that would overshoot. Without a width the
    /// value is used as it stands.
    pub(crate) fn expand(&self, width: Option<usize>) -> String {
        match width {
            Some(width) => self
                .value
                .graphemes(true)
                .cycle()
                .scan(0usize, |used, grapheme| {
                    *used += Grapheme(grapheme).width();
                    if *used <= width { Some(grapheme) } else { None }
                })
                .collect(),
            None => String::from(&self.value),
        }
    }

    pub fn ansi_string(
        &self,
        width: Option<usize>,
        previous: Option<&AnsiStyle>,
    ) -> AnsiString<'_> {
        let text = self.expand(width);
        match self.style {
            Some(style) => style.to_ansi_style(previous).paint(text),
            None => AnsiString::from(text),
        }
    }
}

#[cfg(test)]
mod fill_seg_tests {
    use super::FillSegment;
    #[test]
    fn expansion_respects_terminal_width() {
        let width: usize = 10;

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
                style: None,
            };
            assert_eq!(*expected, f.expand(Some(width)));
        }
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
    pub(crate) fn text(style: Option<Style>, value: impl Into<String>) -> Self {
        Self::Text(TextSegment {
            style,
            value: value.into(),
        })
    }

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
            segs.push(Self::text(style, s));
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
            Self::Text(text) => text.style.map(|style| style.to_ansi_style(None)),
            Self::Fill(fill) => fill.style.map(|style| style.to_ansi_style(None)),
            Self::LineTerm => None,
        }
    }

    pub(crate) fn set_style_if_empty(&mut self, style: Option<Style>) {
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

    pub(crate) fn value(&self) -> &str {
        match self {
            Self::Fill(fs) => &fs.value,
            Self::Text(ts) => &ts.value,
            Self::LineTerm => LINE_TERMINATOR_STRING,
        }
    }

    pub fn ansi_string(&self, previous: Option<&AnsiStyle>) -> AnsiString<'_> {
        match self {
            Self::Text(text) => text.ansi_string(previous),
            Self::Fill(fill) => fill.ansi_string(None, previous),
            Self::LineTerm => AnsiString::from(LINE_TERMINATOR_STRING),
        }
    }

    pub(crate) fn width_graphemes(&self) -> usize {
        match self {
            Self::Fill(fs) => fs.value.width_graphemes(),
            Self::Text(ts) => ts.value.width_graphemes(),
            Self::LineTerm => 0,
        }
    }
}

const LINE_TERMINATOR: char = '\n';
const LINE_TERMINATOR_STRING: &str = "\n";
