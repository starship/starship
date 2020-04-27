use ansi_term::Style;
use pest::error::Error as PestError;
use rayon::prelude::*;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::iter::FromIterator;

use crate::config::parse_style_string;
use crate::segment::Segment;

use super::model::*;
use super::parser::{parse, Rule};

#[derive(Clone)]
enum VariableValue<'a> {
    Plain(Cow<'a, str>),
    Styled(Vec<Segment>),
    Meta(Vec<FormatElement<'a>>),
}

impl<'a> Default for VariableValue<'a> {
    fn default() -> Self {
        VariableValue::Plain(Cow::Borrowed(""))
    }
}

type VariableMapType<'a> =
    BTreeMap<String, Option<Result<VariableValue<'a>, StringFormatterError>>>;
type StyleVariableMapType<'a> =
    BTreeMap<String, Option<Result<Cow<'a, str>, StringFormatterError>>>;

#[derive(Debug, Clone)]
pub enum StringFormatterError {
    Custom(String),
    Parse(PestError<Rule>),
}

impl fmt::Display for StringFormatterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(error) => write!(f, "{}", error),
            Self::Parse(error) => write!(f, "{}", error),
        }
    }
}

impl Error for StringFormatterError {}

impl From<String> for StringFormatterError {
    fn from(error: String) -> Self {
        StringFormatterError::Custom(error)
    }
}

pub struct StringFormatter<'a> {
    format: Vec<FormatElement<'a>>,
    variables: VariableMapType<'a>,
    style_variables: StyleVariableMapType<'a>,
}

impl<'a> StringFormatter<'a> {
    /// Creates an instance of StringFormatter from a format string
    pub fn new(format: &'a str) -> Result<Self, StringFormatterError> {
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
            .map_err(StringFormatterError::Parse)
    }

    /// Maps variable name to its value
    pub fn map<T, M>(mut self, mapper: M) -> Self
    where
        T: Into<Cow<'a, str>>,
        M: Fn(&str) -> Option<Result<T, StringFormatterError>> + Sync,
    {
        self.variables
            .par_iter_mut()
            .filter(|(_, value)| value.is_none())
            .for_each(|(key, value)| {
                *value = mapper(key).map(|var| var.map(|var| VariableValue::Plain(var.into())));
            });
        self
    }

    /// Maps a meta-variable to a format string containing other variables.
    ///
    /// This function should be called **before** other map methods so that variables can be cached
    /// properly.
    pub fn map_meta<M>(mut self, mapper: M) -> Self
    where
        M: Fn(&str, &BTreeSet<String>) -> Option<&'a str> + Sync,
    {
        let variables = self.get_variables();
        let (variables, style_variables) = self
            .variables
            .iter_mut()
            .filter(|(_, value)| value.is_none())
            .fold(
                (VariableMapType::new(), StyleVariableMapType::new()),
                |(mut v, mut sv), (key, value)| {
                    *value = mapper(key, &variables).map(|format| {
                        StringFormatter::new(format).map(|formatter| {
                            let StringFormatter {
                                format,
                                mut variables,
                                mut style_variables,
                            } = formatter;

                            // Add variables in meta variables to self
                            v.append(&mut variables);
                            sv.append(&mut style_variables);

                            VariableValue::Meta(format)
                        })
                    });

                    (v, sv)
                },
            );

        self.variables.extend(variables);
        self.style_variables.extend(style_variables);

        self
    }

    /// Maps variable name to an array of segments
    pub fn map_variables_to_segments<M>(mut self, mapper: M) -> Self
    where
        M: Fn(&str) -> Option<Result<Vec<Segment>, StringFormatterError>> + Sync,
    {
        self.variables
            .par_iter_mut()
            .filter(|(_, value)| value.is_none())
            .for_each(|(key, value)| {
                *value = mapper(key).map(|var| var.map(VariableValue::Styled));
            });
        self
    }

    /// Maps variable name in a style string to its value
    pub fn map_style<T, M>(mut self, mapper: M) -> Self
    where
        T: Into<Cow<'a, str>>,
        M: Fn(&str) -> Option<Result<T, StringFormatterError>> + Sync,
    {
        self.style_variables
            .par_iter_mut()
            .filter(|(_, value)| value.is_none())
            .for_each(|(key, value)| {
                *value = mapper(key).map(|var| var.map(|var| var.into()));
            });
        self
    }

    /// Parse the format string and consume self.
    pub fn parse(self, default_style: Option<Style>) -> Result<Vec<Segment>, StringFormatterError> {
        fn _parse_textgroup<'a>(
            textgroup: TextGroup<'a>,
            variables: &'a VariableMapType<'a>,
            style_variables: &'a StyleVariableMapType<'a>,
        ) -> Result<Vec<Segment>, StringFormatterError> {
            let style = _parse_style(textgroup.style, style_variables);
            _parse_format(
                textgroup.format,
                style.transpose()?,
                &variables,
                &style_variables,
            )
        }

        fn _parse_style<'a>(
            style: Vec<StyleElement>,
            variables: &'a StyleVariableMapType<'a>,
        ) -> Option<Result<Style, StringFormatterError>> {
            let style_strings = style
                .into_iter()
                .map(|style| match style {
                    StyleElement::Text(text) => Ok(text),
                    StyleElement::Variable(name) => {
                        let variable = variables.get(name.as_ref()).unwrap_or(&None);
                        match variable {
                            Some(style_string) => style_string.clone().map(|string| string),
                            None => Ok("".into()),
                        }
                    }
                })
                .collect::<Result<Vec<Cow<str>>, StringFormatterError>>();
            style_strings
                .map(|style_strings| {
                    let style_string: String =
                        style_strings.iter().flat_map(|s| s.chars()).collect();
                    parse_style_string(&style_string)
                })
                .transpose()
        }

        fn _parse_format<'a>(
            format: Vec<FormatElement<'a>>,
            style: Option<Style>,
            variables: &'a VariableMapType<'a>,
            style_variables: &'a StyleVariableMapType<'a>,
        ) -> Result<Vec<Segment>, StringFormatterError> {
            let results: Result<Vec<Vec<Segment>>, StringFormatterError> = format
                .into_iter()
                .map(|el| {
                    match el {
                        FormatElement::Text(text) => Ok(vec![_new_segment("_text", text, style)]),
                        FormatElement::TextGroup(textgroup) => {
                            let textgroup = TextGroup {
                                format: textgroup.format,
                                style: textgroup.style,
                            };
                            _parse_textgroup(textgroup, &variables, &style_variables)
                        }
                        FormatElement::Variable(name) => variables
                            .get(name.as_ref())
                            .expect("Uncached variable found")
                            .as_ref()
                            .map(|segments| match segments.clone()? {
                                VariableValue::Styled(segments) => Ok(segments
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
                                    .collect()),
                                VariableValue::Plain(text) => {
                                    Ok(vec![_new_segment(name, text, style)])
                                }
                                VariableValue::Meta(format) => {
                                    let formatter = StringFormatter {
                                        format,
                                        variables: _clone_without_meta(variables),
                                        style_variables: style_variables.clone(),
                                    };
                                    formatter.parse(style)
                                }
                            })
                            .unwrap_or_else(|| Ok(Vec::new())),
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
                                Ok(Vec::new())
                            }
                        }
                    }
                })
                .collect();
            Ok(results?.into_iter().flatten().collect())
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
fn _new_segment(
    name: impl Into<String>,
    value: impl Into<String>,
    style: Option<Style>,
) -> Segment {
    Segment {
        _name: name.into(),
        value: value.into(),
        style,
    }
}

fn _clone_without_meta<'a>(variables: &VariableMapType<'a>) -> VariableMapType<'a> {
    VariableMapType::from_iter(variables.iter().map(|(key, value)| {
        let value = match value {
            Some(Ok(value)) => match value {
                VariableValue::Meta(_) => None,
                other => Some(Ok(other.clone())),
            },
            Some(Err(e)) => Some(Err(e.clone())),
            None => None,
        };
        (key.clone(), value)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ansi_term::Color;

    // match_next(result: IterMut<Segment>, value, style)
    macro_rules! match_next {
        ($iter:ident, $value:literal, $($style:tt)+) => {
            let _next = $iter.next().unwrap();
            assert_eq!(_next.value, $value);
            assert_eq!(_next.style, $($style)+);
        }
    }

    fn empty_mapper(_: &str) -> Option<Result<String, StringFormatterError>> {
        None
    }

    #[test]
    fn test_default_style() {
        const FORMAT_STR: &str = "text";
        let style = Some(Color::Red.bold());

        let formatter = StringFormatter::new(FORMAT_STR).unwrap().map(empty_mapper);
        let result = formatter.parse(style).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, "text", style);
    }

    #[test]
    fn test_textgroup_text_only() {
        const FORMAT_STR: &str = "[text](red bold)";
        let formatter = StringFormatter::new(FORMAT_STR).unwrap().map(empty_mapper);
        let result = formatter.parse(None).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, "text", Some(Color::Red.bold()));
    }

    #[test]
    fn test_variable_only() {
        const FORMAT_STR: &str = "$var1";

        let formatter = StringFormatter::new(FORMAT_STR)
            .unwrap()
            .map(|variable| match variable {
                "var1" => Some(Ok("text1".to_owned())),
                _ => None,
            });
        let result = formatter.parse(None).unwrap();
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
                "style" => Some(Ok("red bold".to_owned())),
                _ => None,
            });
        let result = formatter.parse(None).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, "root", root_style);
    }

    #[test]
    fn test_scoped_variable() {
        const FORMAT_STR: &str = "${env:PWD}";

        let formatter = StringFormatter::new(FORMAT_STR)
            .unwrap()
            .map(|variable| Some(Ok(format!("${{{}}}", variable))));
        let result = formatter.parse(None).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, "${env:PWD}", None);
    }

    #[test]
    fn test_escaped_chars() {
        const FORMAT_STR: &str = r#"\\\[\$text\]\(red bold\)"#;

        let formatter = StringFormatter::new(FORMAT_STR).unwrap().map(empty_mapper);
        let result = formatter.parse(None).unwrap();
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
        let result = formatter.parse(outer_style).unwrap();
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
                "var" => Some(Ok("text".to_owned())),
                _ => None,
            });
        let result = formatter.parse(None).unwrap();
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
                "var" => Some(Ok(vec![
                    _new_segment("_1".to_owned(), "styless".to_owned(), None),
                    _new_segment("_2".to_owned(), "styled".to_owned(), styled_style),
                    _new_segment(
                        "_3".to_owned(),
                        "styled_no_modifier".to_owned(),
                        styled_no_modifier_style,
                    ),
                ])),
                _ => None,
            });
        let result = formatter.parse(None).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, "styless", var_style);
        match_next!(result_iter, "styled", styled_style);
        match_next!(result_iter, "styled_no_modifier", styled_no_modifier_style);
    }

    #[test]
    fn test_meta_variable() {
        const FORMAT_STR: &str = "$all";
        const FORMAT_STR__ALL: &str = "$a$b";

        let formatter = StringFormatter::new(FORMAT_STR)
            .unwrap()
            .map_meta(|var, _| match var {
                "all" => Some(FORMAT_STR__ALL),
                _ => None,
            })
            .map(|var| match var {
                "a" => Some(Ok("$a")),
                "b" => Some(Ok("$b")),
                _ => None,
            });
        let result = formatter.parse(None).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, "$a", None);
        match_next!(result_iter, "$b", None);
    }

    #[test]
    fn test_multiple_mapper() {
        const FORMAT_STR: &str = "$a$b$c";

        let formatter = StringFormatter::new(FORMAT_STR)
            .unwrap()
            .map(|var| match var {
                "a" => Some(Ok("$a")),
                "b" => Some(Ok("$b")),
                _ => None,
            })
            .map(|var| match var {
                "b" => Some(Ok("$B")),
                "c" => Some(Ok("$c")),
                _ => None,
            });
        let result = formatter.parse(None).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, "$a", None);
        match_next!(result_iter, "$b", None);
        match_next!(result_iter, "$c", None);
    }

    #[test]
    fn test_positional() {
        const FORMAT_STR: &str = "($some) should render but ($none) shouldn't";

        let formatter = StringFormatter::new(FORMAT_STR)
            .unwrap()
            .map(|var| match var {
                "some" => Some(Ok("$some")),
                _ => None,
            });
        let result = formatter.parse(None).unwrap();
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
                "some" => Some(Ok("$some")),
                _ => None,
            });
        let result = formatter.parse(None).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, "$some", None);
        match_next!(result_iter, " ", None);
        match_next!(result_iter, " and ", None);
        match_next!(result_iter, " ", None);
        match_next!(result_iter, "$some", None);
    }

    #[test]
    fn test_variable_holder() {
        const FORMAT_STR: &str = "($a [($b) $c](none $s)) $d [t]($t)";
        let expected_variables =
            BTreeSet::from_iter(vec!["a", "b", "c", "d"].into_iter().map(String::from));

        let formatter = StringFormatter::new(FORMAT_STR).unwrap().map(empty_mapper);
        let variables = formatter.get_variables();
        assert_eq!(variables, expected_variables);
    }

    #[test]
    fn test_style_variable_holder() {
        const FORMAT_STR: &str = "($a [($b) $c](none $s)) $d [t]($t)";
        let expected_variables = BTreeSet::from_iter(vec!["s", "t"].into_iter().map(String::from));

        let formatter = StringFormatter::new(FORMAT_STR).unwrap().map(empty_mapper);
        let variables = formatter.get_style_variables();
        assert_eq!(variables, expected_variables);
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

    #[test]
    fn test_variable_error() {
        const FORMAT_STR: &str = "$never$some";
        let never_error = StringFormatterError::Custom("NEVER".to_owned());

        let segments = StringFormatter::new(FORMAT_STR).and_then(|formatter| {
            formatter
                .map(|var| match var {
                    "some" => Some(Ok("some")),
                    "never" => Some(Err(never_error.clone())),
                    _ => None,
                })
                .parse(None)
        });
        assert!(segments.is_err());
    }
}
