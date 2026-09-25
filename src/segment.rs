use crate::{
    config::Style,
    context::Shell,
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
    use nu_ansi_term::Color;

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

    /// Visible width of the segment as rendered by `shell`.
    ///
    /// `shell_prompt_escape` escapes `$`, `` ` ``, `\` (bash) and `%` (zsh)
    /// before the value reaches the segment, so measuring the stored value
    /// overcounts by one per escaped char and starves `$fill`. Unescape for
    /// measurement only; the stored value is still what gets printed.
    pub fn width_graphemes_shell(&self, shell: Shell) -> usize {
        match self {
            Self::Fill(fs) => fs.value.width_graphemes(),
            Self::Text(ts) => unescaped_width(&ts.value, shell),
            Self::LineTerm => 0,
        }
    }
}

/// Width of `value` as rendered by `shell`, discounting escape sequences
/// added by `shell_prompt_escape`.
fn unescaped_width(value: &str, shell: Shell) -> usize {
    if !value.contains(['$', '`', '\\', '%']) {
        return value.width_graphemes();
    }
    match shell {
        // Inverse of shell_prompt_escape for bash: backtick and dollar
        // first, backslash last.
        Shell::Bash => value
            .replace("\\`", "`")
            .replace("\\$", "$")
            .replace("\\\\", "\\")
            .width_graphemes(),
        Shell::Zsh => value.replace("%%", "%").width_graphemes(),
        _ => value.width_graphemes(),
    }
}

#[cfg(test)]
mod unescaped_width_tests {
    use super::*;
    use crate::context::Shell;

    #[test]
    fn bash_escaped_chars_count_as_one() {
        // `$` is stored escaped as `\$` (2 graphemes) but renders as 1.
        let seg = Segment::from_text(None, "\\$".to_string()).remove(0);
        assert_eq!(seg.width_graphemes(), 2);
        assert_eq!(seg.width_graphemes_shell(Shell::Bash), 1);
    }

    #[test]
    fn bash_other_escapes_count_as_one() {
        for stored in ["\\\\", "\\`"] {
            let seg = Segment::from_text(None, stored.to_string()).remove(0);
            assert_eq!(seg.width_graphemes_shell(Shell::Bash), 1);
        }
    }

    #[test]
    fn zsh_percent_counts_as_one() {
        let seg = Segment::from_text(None, "%%".to_string()).remove(0);
        assert_eq!(seg.width_graphemes_shell(Shell::Zsh), 1);
    }

    #[test]
    fn unknown_shell_keeps_stored_width() {
        let seg = Segment::from_text(None, "\\$".to_string()).remove(0);
        assert_eq!(seg.width_graphemes_shell(Shell::Unknown), 2);
    }

    #[test]
    fn plain_text_unaffected() {
        let seg = Segment::from_text(None, "user@machine".to_string()).remove(0);
        assert_eq!(seg.width_graphemes_shell(Shell::Bash), 12);
    }
}

const LINE_TERMINATOR: char = '\n';
const LINE_TERMINATOR_STRING: &str = "\n";
