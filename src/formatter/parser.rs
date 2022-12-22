// Can't rename internal Pest names
#![allow(clippy::upper_case_acronyms)]

use pest::{error::Error, iterators::Pair, Parser};
use pest_derive::*;

use super::model::*;

#[derive(Parser)]
#[grammar = "formatter/spec.pest"]
struct IdentParser;

fn parse_value(value: Pair<Rule>) -> FormatElement {
    match value.as_rule() {
        Rule::text => FormatElement::Text(parse_text(value).into()),
        Rule::variable => FormatElement::Variable(parse_variable(value).into()),
        Rule::textgroup => FormatElement::TextGroup(parse_textgroup(value)),
        Rule::conditional => {
            FormatElement::Conditional(parse_format(value.into_inner().next().unwrap()))
        }
        _ => unreachable!(),
    }
}

fn parse_textgroup(textgroup: Pair<Rule>) -> TextGroup {
    let mut inner_rules = textgroup.into_inner();
    let format = inner_rules.next().unwrap();
    let style = inner_rules.next().unwrap();

    TextGroup {
        format: parse_format(format),
        style: parse_style(style),
    }
}

fn parse_variable(variable: Pair<Rule>) -> &str {
    variable.into_inner().next().unwrap().as_str()
}

fn parse_text(text: Pair<Rule>) -> String {
    text.into_inner()
        .flat_map(|pair| pair.as_str().chars())
        .collect()
}

fn parse_format(format: Pair<Rule>) -> Vec<FormatElement> {
    format.into_inner().map(parse_value).collect()
}

fn parse_style(style: Pair<Rule>) -> Vec<StyleElement> {
    style
        .into_inner()
        .map(|pair| match pair.as_rule() {
            Rule::string => StyleElement::Text(pair.as_str().into()),
            Rule::variable => StyleElement::Variable(parse_variable(pair).into()),
            _ => unreachable!(),
        })
        .collect()
}

pub fn parse(format: &str) -> Result<Vec<FormatElement>, Box<Error<Rule>>> {
    IdentParser::parse(Rule::expression, format)
        .map(|pairs| {
            pairs
                .take_while(|pair| pair.as_rule() != Rule::EOI)
                .map(parse_value)
                .collect()
        })
        .map_err(Box::new)
}
