use crate::{
    config::Style, context::Shell, formatter::string_formatter::shell_prompt_escape,
    print::UnicodeWidthGraphemes,
};
use nu_ansi_term::{AnsiString, Style as AnsiStyle};

/// A piece of a prompt: text in a style, or the end of a line.
///
/// Text keeps any shell escaping the formatter applied. Shell wrappers for ANSI
/// sequences are added when the prompt is written out.
#[derive(Clone, Debug, PartialEq)]
pub enum Segment {
    Styled {
        kind: Kind,
        /// If none, the style of the module showing it.
        style: Option<Style>,
        value: String,
        /// Original unescaped text when shell escaping changed `value`, so
        /// width calculations measure what the terminal displays.
        display: Option<String>,
    },
    LineBreak,
}

/// How a styled segment is painted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Shown as written.
    Text,
    /// Repeated across the width the rest of its line leaves.
    Fill,
}

impl Segment {
    /// Segments showing `value` in `style`, split at its line breaks.
    pub fn from_text(style: Option<Style>, value: impl Into<String>) -> Vec<Self> {
        Self::lines(Kind::Text, style, value.into())
    }

    /// Segments showing shell-escaped `value` in `style`, split at line breaks,
    /// retaining the unescaped text for terminal width calculation when escaping changed it.
    pub fn from_shell_text(style: Option<Style>, value: &str, shell: Shell) -> Vec<Self> {
        let make = |line: &str| {
            let escaped = shell_prompt_escape(line, shell);
            let display = (escaped != line).then(|| line.to_owned());
            Self::Styled {
                kind: Kind::Text,
                style,
                value: escaped,
                display,
            }
        };
        if !value.contains('\n') {
            return vec![make(value)];
        }
        let mut segments = Vec::new();
        for line in value.split('\n') {
            if !segments.is_empty() {
                segments.push(Self::LineBreak);
            }
            segments.push(make(line));
        }
        segments
    }

    /// A segment repeating `value` across the width its line leaves.
    pub fn fill(style: Option<Style>, value: impl Into<String>) -> Self {
        Self::Styled {
            kind: Kind::Fill,
            style,
            value: value.into(),
            display: None,
        }
    }

    fn lines(kind: Kind, style: Option<Style>, value: String) -> Vec<Self> {
        let styled = |value| Self::Styled {
            kind,
            style,
            value,
            display: None,
        };
        if !value.contains('\n') {
            return vec![styled(value)];
        }
        let mut segments = Vec::new();
        for line in value.split('\n') {
            if !segments.is_empty() {
                segments.push(Self::LineBreak);
            }
            segments.push(styled(line.to_owned()));
        }
        segments
    }

    pub fn style(&self) -> Option<AnsiStyle> {
        match self {
            Self::Styled { style, .. } => style.map(|style| style.to_ansi_style(None)),
            Self::LineBreak => None,
        }
    }

    pub const fn set_style_if_empty(&mut self, style: Option<Style>) {
        if let Self::Styled { style: own, .. } = self
            && own.is_none()
        {
            *own = style;
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Self::Styled { value, .. } => value,
            Self::LineBreak => "\n",
        }
    }

    /// The unescaped text as drawn by the terminal (for width measurement).
    pub fn display_value(&self) -> &str {
        match self {
            Self::Styled {
                display: Some(display),
                ..
            } => display,
            Self::Styled { value, .. } => value,
            Self::LineBreak => "\n",
        }
    }

    /// The value in its style, resolved against `previous` and unstretched.
    pub fn ansi_string(&self, previous: Option<&AnsiStyle>) -> AnsiString<'_> {
        match self {
            Self::Styled {
                style: Some(style),
                value,
                ..
            } => style.to_ansi_style(previous).paint(value),
            segment => AnsiString::from(segment.value()),
        }
    }

    pub fn width_graphemes(&self) -> usize {
        match self {
            Self::Styled { .. } => self.display_value().width_graphemes(),
            Self::LineBreak => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_breaks_into_lines() {
        assert_eq!(
            vec![
                Segment::Styled {
                    kind: Kind::Text,
                    style: None,
                    value: "a".to_owned(),
                    display: None,
                },
                Segment::LineBreak,
                Segment::Styled {
                    kind: Kind::Text,
                    style: None,
                    value: String::new(),
                    display: None,
                },
            ],
            Segment::from_text(None, "a\n")
        );
    }

    #[test]
    fn a_style_given_later_fills_in_only_a_missing_one() {
        let red = Some(nu_ansi_term::Color::Red.into());
        let blue = Some(nu_ansi_term::Color::Blue.into());
        let mut unstyled = Segment::from_text(None, "a").remove(0);
        let mut styled = Segment::from_text(red, "b").remove(0);
        unstyled.set_style_if_empty(blue);
        styled.set_style_if_empty(blue);

        assert_eq!(
            blue.map(|style: Style| style.to_ansi_style(None)),
            unstyled.style()
        );
        assert_eq!(
            red.map(|style: Style| style.to_ansi_style(None)),
            styled.style()
        );
    }
}
