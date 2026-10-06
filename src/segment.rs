use crate::{config::Style, print::UnicodeWidthGraphemes};
use nu_ansi_term::{AnsiString, Style as AnsiStyle};

/// A piece of a prompt: text in a style, or the end of a line.
///
/// Text is stored as the terminal should show it. Escaping it for a shell's
/// prompt variable happens only when a painted prompt is written out.
#[derive(Clone, Debug, PartialEq)]
pub enum Segment {
    Styled {
        kind: Kind,
        /// If none, the style of the module showing it.
        style: Option<Style>,
        value: String,
    },
    LineBreak,
}

/// What the text of a styled segment is, which decides how it is painted and
/// how a shell receives it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Shown as written.
    Text,
    /// Handed to a shell exactly as written, so that the shell may expand it:
    /// the output of a custom module with `unsafe_no_escape`.
    Verbatim,
    /// Repeated across whatever width the rest of its line leaves.
    Fill,
}

impl Segment {
    /// Segments showing `value` in `style`, with a line break wherever it
    /// holds one.
    pub fn from_text(style: Option<Style>, value: impl Into<String>) -> Vec<Self> {
        Self::lines(Kind::Text, style, value.into())
    }

    /// Segments a shell receives unescaped; see [`Kind::Verbatim`].
    pub fn verbatim(style: Option<Style>, value: impl Into<String>) -> Vec<Self> {
        Self::lines(Kind::Verbatim, style, value.into())
    }

    /// A segment repeating `value` across the width its line leaves.
    pub fn fill(style: Option<Style>, value: impl Into<String>) -> Self {
        Self::Styled {
            kind: Kind::Fill,
            style,
            value: value.into(),
        }
    }

    fn lines(kind: Kind, style: Option<Style>, value: String) -> Vec<Self> {
        let styled = |value| Self::Styled { kind, style, value };
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

    pub fn set_style_if_empty(&mut self, style: Option<Style>) {
        if let Self::Styled { style: own, .. } = self {
            *own = own.or(style);
        }
    }

    pub fn value(&self) -> &str {
        match self {
            Self::Styled { value, .. } => value,
            Self::LineBreak => "\n",
        }
    }

    /// The value in its style, resolved against `previous`: what the segment
    /// paints as, unstretched.
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
            Self::Styled { value, .. } => value.width_graphemes(),
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
                    kind: Kind::Verbatim,
                    style: None,
                    value: "a".to_owned()
                },
                Segment::LineBreak,
                Segment::Styled {
                    kind: Kind::Verbatim,
                    style: None,
                    value: String::new()
                },
            ],
            Segment::verbatim(None, "a\n")
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
