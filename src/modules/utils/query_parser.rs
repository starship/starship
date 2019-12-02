use crate::segment::Segment;
use ansi_term::Style;
use serde_json::Value;
use spongy::{parse, Element, Wrapper};
use std::error::Error;

use crate::config::parse_style_string;

/// Parse query from a module query string
///
/// ## Example
/// ```rust
/// let (module_name, query) = parse_query("rust?style=red bold");
/// assert_eq!(module_name, "rust");
/// assert_eq!(query.unwrap().get("style"), Some("red bold"));
/// ```
pub fn parse_query(qstr: &str) -> (&str, Option<Value>) {
    qstr.find('?')
        .and_then(|index| {
            let (module_name, query_with_qmark) = qstr.split_at(index);
            let query = queryst::parse(query_with_qmark.get(1..).unwrap()).ok();
            Some((module_name, query))
        })
        .unwrap_or((qstr, None))
}

/// Helper function to get style from query
pub fn get_style_from_query(query: &Option<Value>) -> Option<Style> {
    query
        .as_ref()
        .and_then(|q| parse_style_string(q.get("style")?.as_str()?))
}

pub fn get_percentage_char() -> &'static str {
    let shell = std::env::var("STARSHIP_SHELL").unwrap_or_default();
    match shell.as_str() {
        "zsh" => "%%", // % is an escape in zsh, see PROMPT in `man zshmisc`
        "powershell" => "`%",
        _ => "%",
    }
}

pub fn get_styled(query: &Option<Value>) -> Option<Segment> {
    let value = query.as_ref()?.get("value")?.as_str()?.to_string();
    let style = get_style_from_query(query);

    Some(Segment {
        _name: "_styled".to_string(),
        value,
        style,
    })
}

pub fn format_segments<M>(
    format_str: &str,
    default_style: Option<Style>,
    mapper: M,
) -> Result<Vec<Segment>, Box<dyn Error>>
where
    M: Fn(&str, Option<Value>) -> Option<Segment>,
{
    let segments = parse(format_str)?
        .iter()
        .map(|el: &Element| -> Option<Segment> {
            match el {
                Element::Text(t) => Some(Segment {
                    _name: "_".to_string(),
                    style: default_style,
                    value: t.to_owned().to_string(),
                }),
                Element::Wrapped(item) => match item.wrapper {
                    Wrapper::DollarCurly => {
                        let (segment_name, query) = parse_query(item.text);
                        match segment_name {
                            "%" => {
                                let style = get_style_from_query(&query).or(default_style);
                                Some(Segment {
                                    _name: "%".to_string(),
                                    value: get_percentage_char().to_string(),
                                    style,
                                })
                            }
                            "styled" => get_styled(&query),
                            _ => mapper(segment_name, query),
                        }
                    }
                    _ => None,
                },
            }
        })
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    Ok(segments)
}
