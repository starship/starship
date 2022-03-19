use crate::print::{Grapheme, UnicodeWidthGraphemes};
use owo_colors::Style;
use std::fmt;
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
    // Returns the ANSIString of the segment value
    fn ansi_string(&self) -> String {
        match self.style {
            Some(style) => style.style(&self.value).to_string(),
            None => self.value.clone(),
        }
    }

    /// Appends another text segment, keeping the style of this segment
    pub fn append(&mut self, suffix: TextSegment) {
        self.value.push_str(&suffix.value)
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
    // Returns the String of the segment value with ANSI sequences, not including its prefix and suffix
    pub fn ansi_string(&self, width: Option<usize>) -> String {
        let s = match width {
            Some(w) => self
                .value
                .graphemes(true)
                .cycle()
                .scan(0usize, |len, g| {
                    *len += Grapheme(g).width();
                    if *len <= w {
                        Some(g)
                    } else {
                        None
                    }
                })
                .collect::<String>(),
            None => String::from(&self.value),
        };

        match self.style {
            Some(style) => style.style(s).to_string(),
            None => s,
        }
    }
}

#[cfg(test)]
mod fill_seg_tests {
    use super::FillSegment;
    use owo_colors::Style;

    #[test]
    fn ansi_string_width() {
        let width: usize = 10;
        let style = Style::new().blue().bold();

        let inputs = vec![
            (".", ".........."),
            (".:", ".:.:.:.:.:"),
            ("-:-", "-:--:--:--"),
            ("🟦", "🟦🟦🟦🟦🟦"),
            ("🟢🔵🟡", "🟢🔵🟡🟢🔵"),
        ];

        for (text, expected) in inputs.iter() {
            let f = FillSegment {
                value: String::from(*text),
                style: Some(style),
            };
            let actual = f.ansi_string(Some(width));
            assert_eq!(style.style(*expected).to_string(), actual);
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
    /// Checks if a given segment is a text segment
    pub fn is_text_segment(&self) -> bool {
        matches!(self, Segment::Text(_))
    }

    /// Creates new segments from a text with a style; breaking out LineTerminators.
    pub fn from_text<T>(style: Option<Style>, value: T) -> Vec<Segment>
    where
        T: Into<String>,
    {
        let mut segs: Vec<Segment> = Vec::new();
        value.into().split(LINE_TERMINATOR).for_each(|s| {
            if !segs.is_empty() {
                segs.push(Segment::LineTerm)
            }
            segs.push(Segment::Text(TextSegment {
                value: String::from(s),
                style,
            }))
        });
        segs
    }

    /// Creates a new fill segment
    pub fn fill<T>(style: Option<Style>, value: T) -> Self
    where
        T: Into<String>,
    {
        Segment::Fill(FillSegment {
            style,
            value: value.into(),
        })
    }

    pub fn style(&self) -> Option<Style> {
        match self {
            Segment::Fill(fs) => fs.style,
            Segment::Text(ts) => ts.style,
            Segment::LineTerm => None,
        }
    }

    pub fn set_style_if_empty(&mut self, style: Option<Style>) {
        match self {
            Segment::Fill(fs) => {
                if fs.style.is_none() {
                    fs.style = style
                }
            }
            Segment::Text(ts) => {
                if ts.style.is_none() {
                    ts.style = style
                }
            }
            Segment::LineTerm => {}
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Segment::Fill(fs) => &fs.value,
            Segment::Text(ts) => &ts.value,
            Segment::LineTerm => LINE_TERMINATOR_STRING,
        }
    }

    // Returns the ANSIString of the segment value, not including its prefix and suffix
    pub fn ansi_string(&self) -> String {
        match self {
            Segment::Fill(fs) => fs.ansi_string(None),
            Segment::Text(ts) => ts.ansi_string(),
            Segment::LineTerm => LINE_TERMINATOR_STRING.to_string(),
        }
    }

    pub fn width_graphemes(&self) -> usize {
        match self {
            Segment::Fill(fs) => fs.value.width_graphemes(),
            Segment::Text(ts) => ts.value.width_graphemes(),
            Segment::LineTerm => 0,
        }
    }
}

const LINE_TERMINATOR: char = '\n';
const LINE_TERMINATOR_STRING: &str = "\n";

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ansi_string())
    }
}
