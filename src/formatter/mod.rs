use crate::config::{parse_style_string, SegmentConfig};
use ansi_term::Style;
use pest::{error::Error, iterators::Pair, Parser};
use rayon::prelude::*;
use std::borrow::Cow;

#[derive(Parser)]
#[grammar = "formatter/spec.pest"]
struct IdentParser;

pub struct StringFormatter<'a, M>
where
    M: Fn(&str) -> Option<String>,
{
    format: &'a str,
    mapper: M,
}

impl<'a, M> StringFormatter<'a, M>
where
    M: Fn(&str) -> Option<String>,
{
    /// Creates an instance of StringFormatter.
    pub fn new(format: &'a str, mapper: M) -> Self {
        Self { format, mapper }
    }

    fn _new_segment(&self, value: String, style: Option<Style>) -> SegmentConfig<'a> {
        SegmentConfig {
            value: Cow::Owned(value),
            style,
        }
    }

    fn _parse_textgroup(
        &self,
        textgroup: Pair<Rule>,
    ) -> Result<Vec<SegmentConfig<'a>>, Error<Rule>> {
        let mut inner_rules = textgroup.into_inner();
        let format = inner_rules.next().unwrap();
        let style_str = inner_rules.next().unwrap().as_str();
        let style = parse_style_string(style_str);
        let mut results: Vec<SegmentConfig<'a>> = Vec::new();

        for pair in format.into_inner() {
            match pair.as_rule() {
                Rule::text => results.push(self._new_segment(self._parse_text(pair), style)),
                Rule::variable => {
                    results.push(self._new_segment(self._parse_variable(pair), style))
                }
                Rule::textgroup => results.extend(self._parse_textgroup(pair)?),
                _ => unreachable!(),
            }
        }

        Ok(results)
    }

    fn _parse_variable(&self, variable: Pair<Rule>) -> String {
        (self.mapper)(variable.into_inner().next().unwrap().as_str())
            .unwrap_or_else(|| String::new())
    }

    fn _parse_text(&self, text: Pair<Rule>) -> String {
        text.as_str().to_owned()
    }

    /// Parse the format string.
    pub fn parse(
        &self,
        default_style: Option<Style>,
    ) -> Result<Vec<SegmentConfig<'a>>, Error<Rule>> {
        let pairs = IdentParser::parse(Rule::expression, self.format)?;
        let mut results: Vec<SegmentConfig<'a>> = Vec::new();

        // Lifetime of SegmentConfig is the same as self.format
        for pair in pairs.take_while(|pair| pair.as_rule() != Rule::EOI) {
            match pair.as_rule() {
                Rule::text => {
                    results.push(self._new_segment(self._parse_text(pair), default_style))
                }
                Rule::variable => {
                    results.push(self._new_segment(self._parse_variable(pair), default_style))
                }
                Rule::textgroup => results.extend(self._parse_textgroup(pair)?),
                _ => unreachable!(),
            }
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ansi_term::Color;

    // match_next(result: Iter<SegmentConfig>, value, style)
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

        let formatter = StringFormatter::new(FORMAT_STR, empty_mapper);
        let result = formatter.parse(style).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, "text", style);
    }

    #[test]
    fn test_textgroup_text_only() {
        const FORMAT_STR: &str = "[text](red bold)";
        let formatter = StringFormatter::new(FORMAT_STR, empty_mapper);
        let result = formatter.parse(None).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, "text", Some(Color::Red.bold()));
    }

    #[test]
    fn test_variable_only() {
        const FORMAT_STR: &str = "$var1";

        fn mapper(variable: &str) -> Option<String> {
            match variable {
                "var1" => Some("text1".to_owned()),
                _ => None,
            }
        }

        let formatter = StringFormatter::new(FORMAT_STR, mapper);
        let result = formatter.parse(None).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, "text1", None);
    }

    #[test]
    fn test_escaped_chars() {
        const FORMAT_STR: &str = r#"\[\$text\]\(red bold\)"#;

        let formatter = StringFormatter::new(FORMAT_STR, empty_mapper);
        let result = formatter.parse(None).unwrap();
        let mut result_iter = result.iter();
        match_next!(result_iter, r#"\[\$text\]\(red bold\)"#, None);
    }

    #[test]
    fn test_nested_textgroup() {
        const FORMAT_STR: &str = "outer [middle [inner](blue)](red bold)";
        let outer_style = Some(Color::Green.normal());
        let middle_style = Some(Color::Red.bold());
        let inner_style = Some(Color::Blue.normal());

        let formatter = StringFormatter::new(FORMAT_STR, empty_mapper);
        let result = formatter.parse(outer_style).unwrap();
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

            let formatter = StringFormatter::new(FORMAT_STR, empty_mapper);
            assert!(formatter.parse(None).is_err());
        }
        // Dollar without variable
        {
            const FORMAT_STR: &str = "$ ";

            let formatter = StringFormatter::new(FORMAT_STR, empty_mapper);
            assert!(formatter.parse(None).is_err());
        }
    }
}
