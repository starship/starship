use crate::{
    config::{Style, StyleRefs},
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
    fn ansi_string(&self, style_refs: Option<StyleRefs>) -> AnsiString<'_> {
        match self.style {
            Some(style) => style.to_ansi_style(style_refs).paint(&self.value),
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
    pub fn ansi_string(
        &self,
        width: Option<usize>,
        style_refs: Option<StyleRefs>,
    ) -> Vec<AnsiString<'_>> {
        self.ansi_string_with_width(width, style_refs).0
    }

    // Returns the AnsiString of the segment value, not including its prefix and suffix, and the width of the resulting string
    pub fn ansi_string_with_width(
        &self,
        width: Option<usize>,
        style_refs: Option<StyleRefs>,
    ) -> (Vec<AnsiString<'_>>, usize) {
        let values_with_width = self
            .value
            .graphemes(true)
            .map(|g| {
                let w = Grapheme(g).width();
                (g, w)
            })
            .collect::<Vec<(&str, usize)>>();
        let mut len = 0;
        let graphemes = match width {
            Some(total_w) => values_with_width
                .into_iter()
                .cycle()
                .take_while(|(_, grapheme_width)| {
                    if len + grapheme_width <= total_w {
                        len += grapheme_width;
                        true
                    } else {
                        false
                    }
                })
                .collect::<Vec<(&str, usize)>>(),
            None => {
                len = values_with_width.iter().map(|(_, w)| *w).sum::<usize>();
                values_with_width
            }
        };

        let s = graphemes.iter().map(|(g, _)| *g).collect::<String>();
        let output = match self.style {
            Some(style) if !style.has_ref_color() || style_refs.is_none() => {
                let ansi_style = style.to_ansi_style(style_refs);
                vec![ansi_style.paint(s)]
            }
            Some(style) => {
                graphemes
                    .iter()
                    .scan(style_refs, |style_ref_cursor, (g, _)| {
                        let ansi_style = style.to_ansi_style(*style_ref_cursor);

                        // Update the cursor for the next grapheme.
                        *style_ref_cursor =
                            style_ref_cursor.map(|refs| refs.with_prev(Some(ansi_style)));

                        Some(ansi_style.paint(*g))
                    })
                    .collect()
            }
            None => vec![AnsiString::from(s)],
        };
        (output, len)
    }
}

#[cfg(test)]
mod fill_seg_tests {
    use super::FillSegment;
    use crate::config::{StyleRefs, parse_style_string};
    use nu_ansi_term::Style as AnsiStyle;
    use nu_ansi_term::{AnsiStrings, Color};

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
            let actual_combined = AnsiStrings(&actual).to_string();
            assert_eq!(style.paint(*expected).to_string(), actual_combined);
        }
    }

    #[test]
    fn ansi_string_with_refs_uses_next_style_for_single_grapheme() {
        let fill = FillSegment {
            value: ".".to_string(),
            style: parse_style_string("fg:next_bg", None),
        };

        let refs = StyleRefs::new(
            Some(AnsiStyle::new().fg(Color::Red).on(Color::Yellow)),
            Some(AnsiStyle::new().fg(Color::Blue).on(Color::Cyan)),
        );

        let rendered = fill.ansi_string(Some(1), Some(refs));
        assert_eq!(
            rendered
                .last()
                .and_then(|s| Some(s.style_ref().foreground))
                .flatten(),
            Some(Color::Cyan)
        );
    }

    #[test]
    fn ansi_string_with_refs_keeps_last_grapheme_connected_to_next_style() {
        let fill = FillSegment {
            value: ".".to_string(),
            style: parse_style_string("fg:prev_fg bg:next_fg", None),
        };

        let refs = StyleRefs::new(
            Some(AnsiStyle::new().fg(Color::Red)),
            Some(AnsiStyle::new().fg(Color::Blue)),
        );

        let rendered = fill.ansi_string(Some(2), Some(refs));
        assert_eq!(
            rendered
                .last()
                .and_then(|s| Some(s.style_ref().foreground))
                .flatten(),
            Some(Color::Red)
        );
        assert_eq!(
            rendered
                .last()
                .and_then(|s| Some(s.style_ref().background))
                .flatten(),
            Some(Color::Blue)
        );
    }

    #[test]
    fn ansi_string_with_refs_applies_fill_striping() {
        let fill = FillSegment {
            value: ".:".to_string(),
            style: parse_style_string("fg:prev_fg", None),
        };

        let refs = StyleRefs::new(
            Some(AnsiStyle::new().fg(Color::Red)),
            Some(AnsiStyle::new().fg(Color::Blue)),
        );

        let rendered = fill.ansi_string(Some(4), Some(refs));
        let rendered_str = AnsiStrings(&rendered).to_string();

        let expected = AnsiStyle::new().fg(Color::Red).paint(".:.:").to_string();

        assert_eq!(rendered_str, expected);
    }

    #[test]
    fn ansi_string_with_refs_striping_handles_odd_width_tail() {
        let fill = FillSegment {
            value: ".:".to_string(),
            style: parse_style_string("fg:next_fg", None),
        };

        let refs = StyleRefs::new(
            Some(AnsiStyle::new().fg(Color::Red)),
            Some(AnsiStyle::new().fg(Color::Blue)),
        );

        let rendered = fill.ansi_string(Some(3), Some(refs));
        let rendered_str = AnsiStrings(&rendered).to_string();

        // The last grapheme should stay connected to the base next style.
        let expected = AnsiStyle::new().fg(Color::Blue).paint(".:.").to_string();

        assert_eq!(rendered_str, expected);
    }

    #[test]
    fn ansi_string_with_refs_next_only_without_next_style_falls_back_cleanly() {
        let fill = FillSegment {
            value: ".:".to_string(),
            style: parse_style_string("fg:next_fg", None),
        };

        let rendered = fill.ansi_string(Some(4), Some(StyleRefs::new(None, None)));

        assert_eq!(
            rendered
                .last()
                .and_then(|s| Some(s.style_ref().foreground))
                .flatten(),
            None
        );
    }

    #[test]
    fn ansi_string_with_refs_uses_self_for_missing_next_until_last_grapheme() {
        let fill = FillSegment {
            value: ".:".to_string(),
            style: parse_style_string("bg:next_bg", None),
        };

        let refs = StyleRefs::new(Some(AnsiStyle::new().on(Color::Red)), None);
        let rendered = fill.ansi_string(Some(2), Some(refs));
        let rendered_str = AnsiStrings(&rendered).to_string();

        let expected = ".:";
        assert_eq!(rendered_str, expected);
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
    pub fn ansi_string(&self, style_refs: Option<StyleRefs>) -> AnsiString<'_> {
        match self {
            Self::Fill(fs) => AnsiString::from(
                fs.ansi_string(None, style_refs)
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<String>(),
            ),
            Self::Text(ts) => ts.ansi_string(style_refs),
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
