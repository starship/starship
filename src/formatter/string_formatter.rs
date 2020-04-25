use ansi_term::Style;
use pest::error::Error;
use rayon::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;

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
                // Cache all variables
                let variables = VariableMapType::from_iter(
                    format
                        .get_variables()
                        .into_iter()
                        .map(|key| (key.to_string(), None))
                        .collect::<Vec<(String, Option<_>)>>(),
                );
                let style_variables = StyleVariableMapType::from_iter(
                    format
                        .get_style_variables()
                        .into_iter()
                        .map(|key| (key.to_string(), None))
                        .collect::<Vec<(String, Option<_>)>>(),
                );
                (format, variables, style_variables)
            })
            .map(|(format, variables, style_variables)| Self {
                format,
                variables,
                style_variables,
            })
    }

    /// Maps variable name to its value
    pub fn map<T: Into<String>>(mut self, mapper: impl Fn(&str) -> Option<T> + Sync) -> Self {
        self.variables.par_iter_mut().for_each(|(key, value)| {
            *value = mapper(key).map(|var| var.into()).map(VariableValue::Plain);
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
                        .and_then(|segments| {
                            Some(match segments.clone()? {
                                VariableValue::Styled(segments) => segments
                                    .into_iter()
                                    .map(|mut segment| {
                                        // Derive upper style if the style of segments are none.
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
                            })
                        })
                        .unwrap_or_default(),
                    FormatElement::Positional(format) => {
                        // Show the positional format string if all the variables inside are not
                        // none.
                        let should_show: bool = format.get_variables().iter().any(|var| {
                            variables
                                .get(var.as_ref())
                                .map(|segments| segments.is_some())
                                .unwrap_or(false)
                        });

                        if should_show {
                            _parse_format(format, style, variables, style_variables)
                        } else {
                            Vec::new()
                        }
                    }
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

impl<'a> VariableHolder<String> for StringFormatter<'a> {
    fn get_variables(&self) -> BTreeSet<String> {
        BTreeSet::from_iter(self.variables.keys().cloned())
    }
}

impl<'a> StyleVariableHolder<String> for StringFormatter<'a> {
    fn get_style_variables(&self) -> BTreeSet<String> {
        BTreeSet::from_iter(self.style_variables.keys().cloned())
    }
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
    fn test_positional() {
        const FORMAT_STR: &str = "($some) should render but ($none) shouldn't";

        let formatter = StringFormatter::new(FORMAT_STR)
            .unwrap()
            .map(|var| match var {
                "some" => Some("$some"),
                _ => None,
            });
        let result = formatter.parse(None);
        let mut result_iter = result.iter();
        match_next!(result_iter, "$some", None);
        match_next!(result_iter, " should render but ", None);
        match_next!(result_iter, " shouldn't", None);
    }

    #[test]
    fn test_nested_positional() {
        const FORMAT_STR: &str = "($some ($none)) and ($none ($some))";

        let formatter = StringFormatter::new(FORMAT_STR)
            .unwrap()
            .map(|var| match var {
                "some" => Some("$some"),
                _ => None,
            });
        let result = formatter.parse(None);
        let mut result_iter = result.iter();
        match_next!(result_iter, "$some", None);
        match_next!(result_iter, " ", None);
        match_next!(result_iter, " and ", None);
        match_next!(result_iter, " ", None);
        match_next!(result_iter, "$some", None);
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
