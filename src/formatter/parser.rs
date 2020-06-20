use pest::{error::Error, iterators::Pair, Parser};

use super::model::*;

#[derive(Parser)]
#[grammar = "formatter/spec.pest"]
struct IdentParser;

fn _parse_value(value: Pair<Rule>) -> FormatElement {
    match value.as_rule() {
        Rule::text => FormatElement::Text(_parse_text(value).into()),
        Rule::variable => FormatElement::Variable(_parse_variable(value).into()),
        Rule::textgroup => FormatElement::TextGroup(_parse_textgroup(value)),
        Rule::conditional => {
            FormatElement::Conditional(_parse_format(value.into_inner().next().unwrap()))
        }
        _ => unreachable!(),
    }
}

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
    text.into_inner()
        .map(|pair| pair.as_str().chars())
        .flatten()
        .collect()
}

fn _parse_format(format: Pair<Rule>) -> Vec<FormatElement> {
    format.into_inner().map(_parse_value).collect()
}

fn _parse_style(style: Pair<Rule>) -> Vec<StyleElement> {
    style
        .into_inner()
        .map(|pair| match pair.as_rule() {
            Rule::string => StyleElement::Text(pair.as_str().into()),
            Rule::variable => StyleElement::Variable(_parse_variable(pair).into()),
            _ => unreachable!(),
        })
        .collect()
}

pub fn parse(format: &str) -> Result<Vec<FormatElement>, Error<Rule>> {
    IdentParser::parse(Rule::expression, format).map(|pairs| {
        pairs
            .take_while(|pair| pair.as_rule() != Rule::EOI)
            .map(_parse_value)
            .collect()
    })
}
