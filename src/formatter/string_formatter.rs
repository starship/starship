use ansi_term::Style;
use pest::error::Error;
use rayon::prelude::*;
use std::collections::BTreeMap;

use crate::config::parse_style_string;
use crate::segment::Segment;

use super::model::*;
use super::parser::{parse, Rule};

#[derive(Clone)]
enum VariableValue {
    Plain(String),
    Styled(Vec<Segment>),
}

impl Default for VariableValue {
    fn default() -> Self {
        VariableValue::Plain(String::new())
    }
}

type VariableMapType = BTreeMap<String, Option<VariableValue>>;
type StyleVariableMapType = BTreeMap<String, Option<String>>;

pub struct StringFormatter<'a> {
    format: Vec<FormatElement<'a>>,
    variables: VariableMapType,
    style_variables: StyleVariableMapType,
}

impl<'a> StringFormatter<'a> {
    /// Creates an instance of StringFormatter from a format string
    pub fn new(format: &'a str) -> Result<Self, Error<Rule>> {
        parse(format)
            .map(|format| {
                let (variables, style_variables) = _get_variables(&format);
                (format, variables, style_variables)
            })
            .map(|(format, variables, style_variables)| Self {
                format,
                variables,
                style_variables,
            })
    }

    /// Maps variable name to its value
    pub fn map(mut self, mapper: impl Fn(&str) -> Option<String> + Sync) -> Self {
        self.variables.par_iter_mut().for_each(|(key, value)| {
            *value = mapper(key).map(VariableValue::Plain);
        });
        self
    }

    /// Maps variable name to an array of segments
    pub fn map_variables_to_segments(
        mut self,
        mapper: impl Fn(&str) -> Option<Vec<Segment>> + Sync,
    ) -> Self {
        self.variables.par_iter_mut().for_each(|(key, value)| {
            *value = mapper(key).map(VariableValue::Styled);
        });
        self
    }

    /// Maps variable name in a style string to its value
    pub fn map_style(mut self, mapper: impl Fn(&str) -> Option<String> + Sync) -> Self {
        self.style_variables
            .par_iter_mut()
            .for_each(|(key, value)| {
                *value = mapper(key);
            });
        self
    }

    /// Parse the format string and consume self.
    pub fn parse(self, default_style: Option<Style>) -> Vec<Segment> {
        fn _parse_textgroup<'a>(
            textgroup: TextGroup<'a>,
            variables: &'a VariableMapType,
            style_variables: &'a StyleVariableMapType,
        ) -> Vec<Segment> {
            let style = _parse_style(textgroup.style, style_variables);
            _parse_format(textgroup.format, style, &variables, &style_variables)
        }

        fn _parse_style<'a>(
            style: Vec<StyleElement>,
            variables: &'a StyleVariableMapType,
        ) -> Option<Style> {
            let style_string = style
                .iter()
                .flat_map(|style| match style {
                    StyleElement::Text(text) => text.as_ref().chars(),
                    StyleElement::Variable(name) => {
                        let variable = variables.get(name.as_ref()).unwrap_or(&None);
                        match variable {
                            Some(style_string) => style_string.chars(),
                            None => "".chars(),
                        }
                    }
                })
                .collect::<String>();
            parse_style_string(&style_string)
        }

        fn _parse_format<'a>(
            format: Vec<FormatElement<'a>>,
            style: Option<Style>,
            variables: &'a VariableMapType,
            style_variables: &'a StyleVariableMapType,
        ) -> Vec<Segment> {
            format
                .into_iter()
                .flat_map(|el| match el {
                    FormatElement::Text(text) => {
                        vec![_new_segment("_text".into(), text.into_owned(), style)]
                    }
                    FormatElement::TextGroup(textgroup) => {
                        let textgroup = TextGroup {
                            format: textgroup.format,
                            style: textgroup.style,
                        };
                        _parse_textgroup(textgroup, &variables, &style_variables)
                    }
                    FormatElement::Variable(name) => variables
                        .get(name.as_ref())
                        .map(|segments| {
                            let value = segments.clone().unwrap_or_default();
                            match value {
                                VariableValue::Styled(segments) => segments
                                    .into_iter()
                                    .map(|mut segment| {
                                        if !segment.has_style() {
                                            if let Some(style) = style {
                                                segment.set_style(style);
                                            }
                                        }
                                        segment
                                    })
                                    .collect(),
                                VariableValue::Plain(text) => {
                                    vec![_new_segment(name.to_string(), text, style)]
                                }
                            }
                        })
                        .unwrap_or_default(),
                })
                .collect()
        }

        _parse_format(
            self.format,
            default_style,
            &self.variables,
            &self.style_variables,
        )
    }
}

/// Extract variable names from an array of `FormatElement` into a `BTreeMap`
fn _get_variables<'a>(format: &[FormatElement<'a>]) -> (VariableMapType, StyleVariableMapType) {
    let mut variables: VariableMapType = Default::default();
    let mut style_variables: StyleVariableMapType = Default::default();

    fn _push_variables_from_textgroup<'a>(
        variables: &mut VariableMapType,
        style_variables: &mut StyleVariableMapType,
        textgroup: &'a TextGroup<'a>,
    ) {
        for el in &textgroup.format {
            match el {
                FormatElement::Variable(name) => _push_variable(variables, name.as_ref()),
                FormatElement::TextGroup(textgroup) => {
                    _push_variables_from_textgroup(variables, style_variables, &textgroup)
                }
                _ => {}
            }
        }
        for el in &textgroup.style {
            if let StyleElement::Variable(name) = el {
                _push_style_variable(style_variables, name.as_ref())
            }
        }
    }

    fn _push_variable<'a>(variables: &mut VariableMapType, name: &'a str) {
        variables.insert(name.to_owned(), None);
    }

    fn _push_style_variable<'a>(style_variables: &mut StyleVariableMapType, name: &'a str) {
        style_variables.insert(name.to_owned(), None);
    }

    for el in format {
        match el {
            FormatElement::Variable(name) => _push_variable(&mut variables, name.as_ref()),
            FormatElement::TextGroup(textgroup) => {
                _push_variables_from_textgroup(&mut variables, &mut style_variables, &textgroup)
            }
            _ => {}
        }
    }

    (variables, style_variables)
}

/// Helper function to create a new segment
fn _new_segment(name: String, value: String, style: Option<Style>) -> Segment {
    Segment {
        _name: name,
        value,
        style,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ansi_term::Color;

    // match_next(result: Iter<Segment>, value, style)
    macro_rules! match_next {
        ($iter:ident, $value:literal, $($style:tt)+) => {
            let _next = $iter.next().unwrap();
            assert_eq!(_next.value, $value);
            assert_eq!(_next.style, $($style)+);
        }
    }

    fn empty_mapper(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn test_default_style() {
        const FORMAT_STR: &str = "text";
        let style = Some(Color::Red.bold());

        let formatter = StringFormatter::new(FORMAT_STR).unwrap().map(empty_mapper);
        let result = formatter.parse(style);
        let mut result_iter = result.iter();
        match_next!(result_iter, "text", style);
    }

    #[test]
    fn test_textgroup_text_only() {
        const FORMAT_STR: &str = "[text](red bold)";
        let formatter = StringFormatter::new(FORMAT_STR).unwrap().map(empty_mapper);
        let result = formatter.parse(None);
        let mut result_iter = result.iter();
        match_next!(result_iter, "text", Some(Color::Red.bold()));
    }

    #[test]
    fn test_variable_only() {
        const FORMAT_STR: &str = "$var1";

        let formatter = StringFormatter::new(FORMAT_STR)
            .unwrap()
            .map(|variable| match variable {
                "var1" => Some("text1".to_owned()),
                _ => None,
            });
        let result = formatter.parse(None);
        let mut result_iter = result.iter();
        match_next!(result_iter, "text1", None);
    }

    #[test]
    fn test_variable_in_style() {
        const FORMAT_STR: &str = "[root]($style)";
        let root_style = Some(Color::Red.bold());

        let formatter = StringFormatter::new(FORMAT_STR)
            .unwrap()
            .map_style(|variable| match variable {
                "style" => Some("red bold".to_owned()),
                _ => None,
            });
        let result = formatter.parse(None);
        let mut result_iter = result.iter();
        match_next!(result_iter, "root", root_style);
    }

    #[test]
    fn test_scoped_variable() {
        const FORMAT_STR: &str = "${env:PWD}";

        let formatter = StringFormatter::new(FORMAT_STR)
            .unwrap()
            .map(|variable| Some(format!("${{{}}}", variable)));
        let result = formatter.parse(None);
        let mut result_iter = result.iter();
        match_next!(result_iter, "${env:PWD}", None);
    }

    #[test]
    fn test_escaped_chars() {
        const FORMAT_STR: &str = r#"\\\[\$text\]\(red bold\)"#;

        let formatter = StringFormatter::new(FORMAT_STR).unwrap().map(empty_mapper);
        let result = formatter.parse(None);
        let mut result_iter = result.iter();
        match_next!(result_iter, r#"\[$text](red bold)"#, None);
    }

    #[test]
    fn test_nested_textgroup() {
        const FORMAT_STR: &str = "outer [middle [inner](blue)](red bold)";
        let outer_style = Some(Color::Green.normal());
        let middle_style = Some(Color::Red.bold());
        let inner_style = Some(Color::Blue.normal());

        let formatter = StringFormatter::new(FORMAT_STR).unwrap().map(empty_mapper);
        let result = formatter.parse(outer_style);
        let mut result_iter = result.iter();
        match_next!(result_iter, "outer ", outer_style);
        match_next!(result_iter, "middle ", middle_style);
        match_next!(result_iter, "inner", inner_style);
    }

    #[test]
    fn test_styled_variable_as_text() {
        const FORMAT_STR: &str = "[$var](red bold)";
        let var_style = Some(Color::Red.bold());

        let formatter = StringFormatter::new(FORMAT_STR)
            .unwrap()
            .map(|variable| match variable {
                "var" => Some("text".to_owned()),
                _ => None,
            });
        let result = formatter.parse(None);
        let mut result_iter = result.iter();
        match_next!(result_iter, "text", var_style);
    }

    #[test]
    fn test_styled_variable_as_segments() {
        const FORMAT_STR: &str = "[$var](red bold)";
        let var_style = Some(Color::Red.bold());
        let styled_style = Some(Color::Green.italic());
        let styled_no_modifier_style = Some(Color::Green.normal());

        let formatter = StringFormatter::new(FORMAT_STR)
            .unwrap()
            .map_variables_to_segments(|variable| match variable {
                "var" => Some(vec![
                    _new_segment("_1".to_owned(), "styless".to_owned(), None),
                    _new_segment("_2".to_owned(), "styled".to_owned(), styled_style),
                    _new_segment(
                        "_3".to_owned(),
                        "styled_no_modifier".to_owned(),
                        styled_no_modifier_style,
                    ),
                ]),
                _ => None,
            });
        let result = formatter.parse(None);
        let mut result_iter = result.iter();
        match_next!(result_iter, "styless", var_style);
        match_next!(result_iter, "styled", styled_style);
        match_next!(result_iter, "styled_no_modifier", styled_no_modifier_style);
    }

    #[test]
    fn test_parse_error() {
        // brackets without escape
        {
            const FORMAT_STR: &str = "[";
            assert!(StringFormatter::new(FORMAT_STR).is_err());
        }
        // Dollar without variable
        {
            const FORMAT_STR: &str = "$ ";
            assert!(StringFormatter::new(FORMAT_STR).is_err());
        }
    }
}
