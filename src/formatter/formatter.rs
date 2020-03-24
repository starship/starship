use ansi_term::Style;
use pest::error::Error;
use rayon::prelude::*;
use std::collections::BTreeMap;

use crate::config::parse_style_string;
use crate::segment::Segment;

use super::model::*;
use super::parser::{parse, Rule};

type VariableMapType = BTreeMap<String, Option<Vec<Segment>>>;

pub struct StringFormatter<'a> {
    format: Vec<FormatElement<'a>>,
    variables: VariableMapType,
}

impl<'a> StringFormatter<'a> {
    /// Creates an instance of StringFormatter from a format string.
    pub fn from_str(format: &'a str) -> Result<Self, Error<Rule>> {
        parse(format)
            .map(|format| {
                let variables = _get_variables(&format);
                (format, variables)
            })
            .map(|(format, variables)| Self { format, variables })
    }

    /// Maps variable name to its value
    pub fn map(mut self, mapper: impl Fn(&str) -> Option<String> + Sync) -> Self {
        self.variables.par_iter_mut().for_each(|(key, value)| {
            *value = mapper(key).map(|value| {
                vec![Segment {
                    _name: key.to_string(),
                    value,
                    style: None,
                }]
            });
        });
        self
    }

    /// Maps variable name to an array of segments
    pub fn map_variables_to_segments(
        mut self,
        mapper: impl Fn(&str) -> Option<Vec<Segment>> + Sync,
    ) -> Self {
        self.variables.par_iter_mut().for_each(|(key, value)| {
            *value = mapper(key);
        });
        self
    }

    /// Parse the format string and consume self.
    pub fn parse(self, default_style: Option<Style>) -> Vec<Segment> {
        fn _parse_textgroup<'a>(
            textgroup: TextGroup<'a>,
            variables: &'a VariableMapType,
        ) -> Vec<Segment> {
            let style = _parse_style(textgroup.style);
            _parse_format(textgroup.format, style, &variables)
        }

        fn _parse_style<'a>(style: Vec<StyleElement<'a>>) -> Option<Style> {
            let style_string = style
                .iter()
                .flat_map(|style| match style {
                    StyleElement::Text(text) => text.as_ref().chars(),
                    StyleElement::Variable(variable) => {
                        log::warn!(
                            "Variable `{}` monitored in style string, which is not allowed",
                            &variable
                        );
                        "".chars()
                    }
                })
                .collect::<String>();
            parse_style_string(&style_string)
        }

        fn _parse_format<'a>(
            mut format: Vec<FormatElement<'a>>,
            style: Option<Style>,
            variables: &'a VariableMapType,
        ) -> Vec<Segment> {
            let mut result: Vec<Segment> = Vec::new();

            format.reverse();
            while let Some(el) = format.pop() {
                let mut segments = match el {
                    FormatElement::Text(text) => {
                        vec![_new_segment("_text".into(), text.into_owned(), style)]
                    }
                    FormatElement::TextGroup(textgroup) => {
                        let textgroup = TextGroup {
                            format: textgroup.format,
                            style: textgroup.style,
                        };
                        _parse_textgroup(textgroup, &variables)
                    }
                    FormatElement::Variable(name) => variables
                        .get(name.as_ref())
                        .map(|segments| segments.clone().unwrap_or_else(|| vec![]))
                        .unwrap_or_else(|| vec![]),
                };
                result.append(&mut segments);
            }

            result
        }

        _parse_format(self.format, default_style, &self.variables)
    }
}

fn _get_variables<'a>(format: &'a Vec<FormatElement<'a>>) -> VariableMapType {
    let mut variables: VariableMapType = Default::default();

    fn _push_variables_from_textgroup<'a>(
        variables: &mut VariableMapType,
        textgroup: &'a TextGroup<'a>,
    ) {
        for el in &textgroup.format {
            match el {
                FormatElement::Variable(name) => _push_variable(variables, name.as_ref()),
                FormatElement::TextGroup(textgroup) => {
                    _push_variables_from_textgroup(variables, &textgroup)
                }
                _ => {}
            }
        }
        for el in &textgroup.style {
            if let StyleElement::Variable(name) = el {
                _push_variable(variables, name.as_ref())
            }
        }
    }

    fn _push_variable<'a>(variables: &mut VariableMapType, name: &'a str) {
        variables.insert(name.to_owned(), None);
    }

    for el in format {
        match el {
            FormatElement::Variable(name) => _push_variable(&mut variables, name.as_ref()),
            FormatElement::TextGroup(textgroup) => {
                _push_variables_from_textgroup(&mut variables, &textgroup)
            }
            _ => {}
        }
    }

    variables
}

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

        let formatter = StringFormatter::from_str(FORMAT_STR)
            .unwrap()
            .map(empty_mapper);
        let result = formatter.parse(style);
        let mut result_iter = result.iter();
        match_next!(result_iter, "text", style);
    }

    #[test]
    fn test_textgroup_text_only() {
        const FORMAT_STR: &str = "[text](red bold)";
        let formatter = StringFormatter::from_str(FORMAT_STR)
            .unwrap()
            .map(empty_mapper);
        let result = formatter.parse(None);
        let mut result_iter = result.iter();
        match_next!(result_iter, "text", Some(Color::Red.bold()));
    }

    #[test]
    fn test_variable_only() {
        const FORMAT_STR: &str = "$var1";

        let formatter =
            StringFormatter::from_str(FORMAT_STR)
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
    fn test_escaped_chars() {
        const FORMAT_STR: &str = r#"\\\[\$text\]\(red bold\)"#;

        let formatter = StringFormatter::from_str(FORMAT_STR)
            .unwrap()
            .map(empty_mapper);
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

        let formatter = StringFormatter::from_str(FORMAT_STR)
            .unwrap()
            .map(empty_mapper);
        let result = formatter.parse(outer_style);
        let mut result_iter = result.iter();
        match_next!(result_iter, "outer ", outer_style);
        match_next!(result_iter, "middle ", middle_style);
        match_next!(result_iter, "inner", inner_style);
    }

    #[test]
    fn test_parse_error() {
        // brackets without escape
        {
            const FORMAT_STR: &str = "[";
            assert!(StringFormatter::from_str(FORMAT_STR).is_err());
        }
        // Dollar without variable
        {
            const FORMAT_STR: &str = "$ ";
            assert!(StringFormatter::from_str(FORMAT_STR).is_err());
        }
    }
}
