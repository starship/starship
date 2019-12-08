use pest::{error::Error, Parser};

#[derive(Parser)]
#[grammar = "formatter/spec.pest"]
struct IdentParser;

#[derive(PartialEq, Debug)]
pub enum Wrapper {
    DollarCurly,
}

impl Wrapper {
    fn get_prefix(&self) -> &'static str {
        match self {
            Wrapper::DollarCurly => "${",
        }
    }

    fn get_suffix(&self) -> &'static str {
        match self {
            Wrapper::DollarCurly => "}",
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Item<'a> {
    pub wrapper: Wrapper,
    pub text: &'a str,
}

impl<'a> Item<'a> {
    fn new(wrapper: Wrapper, text: &'a str) -> Item<'a> {
        Item { wrapper, text }
    }
}

#[derive(PartialEq, Debug)]
pub enum Element<'a> {
    Text(&'a str),
    Wrapped(Item<'a>),
}

pub fn parse<'e>(s: &'e str) -> Result<Vec<Element<'e>>, Error<Rule>> {
    let pairs = IdentParser::parse(Rule::expression, s)?;

    let result: Vec<Element<'e>> = pairs
        .take_while(|pair| pair.as_rule() != Rule::EOI)
        .map(|pair| match pair.as_rule() {
            Rule::text => Element::Text(pair.into_inner().next().unwrap().as_str()),
            Rule::dollar_curly => Element::Wrapped(Item::new(
                Wrapper::DollarCurly,
                pair.into_inner().next().unwrap().as_str(),
            )),
            Rule::EOI => Element::Text(""),
            _ => unreachable!(),
        })
        .collect();

    Ok(result)
}

pub fn parse_with<M>(s: &str, mapper: M) -> Result<String, Error<Rule>>
where
    M: Fn(&Item) -> Option<String>,
{
    let result = parse(s)?
        .iter()
        .map(|el: &Element| -> String {
            match el {
                Element::Text(t) => t.to_owned().to_owned(),
                Element::Wrapped(item) => mapper(item).unwrap_or_else(|| {
                    format!(
                        "{}{}{}",
                        item.wrapper.get_prefix(),
                        item.text,
                        item.wrapper.get_suffix()
                    )
                }),
            }
        })
        .collect::<Vec<String>>()
        .join("");

    Ok(result)
}
