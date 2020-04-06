use pest::{error::Error, iterators::Pair, Parser};

use super::model::*;

#[derive(Parser)]
#[grammar = "formatter/spec.pest"]
struct IdentParser;

fn _parse_textgroup(textgroup: Pair<Rule>) -> TextGroup {
    let mut inner_rules = textgroup.into_inner();
    let format = inner_rules.next().unwrap();
    let style = inner_rules.next().unwrap();

    TextGroup {
        format: _parse_format(format),
        style: _parse_style(style),
    }
}

fn _parse_variable(variable: Pair<Rule>) -> &str {
    variable.into_inner().next().unwrap().as_str()
}

fn _parse_text(text: Pair<Rule>) -> String {
    let mut result = String::new();
    for pair in text.into_inner() {
        result.push_str(pair.as_str());
    }
    result
}

fn _parse_format(format: Pair<Rule>) -> Vec<FormatElement> {
    let mut result: Vec<FormatElement> = Vec::new();

    for pair in format.into_inner() {
        match pair.as_rule() {
            Rule::text => result.push(FormatElement::Text(_parse_text(pair).into())),
            Rule::variable => result.push(FormatElement::Variable(_parse_variable(pair).into())),
            Rule::textgroup => result.push(FormatElement::TextGroup(_parse_textgroup(pair))),
            _ => unreachable!(),
        }
    }

    result
}

fn _parse_style(style: Pair<Rule>) -> Vec<StyleElement> {
    let mut result: Vec<StyleElement> = Vec::new();

    for pair in style.into_inner() {
        match pair.as_rule() {
            Rule::text => result.push(StyleElement::Text(_parse_text(pair).into())),
            Rule::variable => result.push(StyleElement::Variable(_parse_variable(pair).into())),
            _ => unreachable!(),
        }
    }

    result
}

pub fn parse(format: &str) -> Result<Vec<FormatElement>, Error<Rule>> {
    let pairs = IdentParser::parse(Rule::expression, format)?;
    let mut result: Vec<FormatElement> = Vec::new();

    // Lifetime of Segment is the same as result
    for pair in pairs.take_while(|pair| pair.as_rule() != Rule::EOI) {
        match pair.as_rule() {
            Rule::text => result.push(FormatElement::Text(_parse_text(pair).into())),
            Rule::variable => result.push(FormatElement::Variable(_parse_variable(pair).into())),
            Rule::textgroup => result.push(FormatElement::TextGroup(_parse_textgroup(pair))),
            _ => unreachable!(),
        }
    }

    Ok(result)
}
