//! Helper module to format values with a custom mapper
//!
//! Intended for usage inside a [`StringFormatter::map_variables_to_segments()`] call.

use super::super::Context;
use crate::formatter::StringFormatter;
use crate::segment::Segment;

pub fn text<F>(
    format_str: &str,
    config_path: &str,
    context: &Context,
    mapper: F,
) -> Option<Vec<Segment>>
where
    F: Fn(&str) -> Option<String> + Send + Sync,
{
    if let Ok(formatter) = StringFormatter::new(format_str) {
        formatter
            .map(|variable| mapper(variable).map(Ok))
            .parse(None, Some(context))
            .ok()
    } else {
        log::warn!("Error parsing format string `{config_path}`");
        None
    }
}

pub fn count(
    format_str: &str,
    config_path: &str,
    context: &Context,
    count: usize,
) -> Option<Vec<Segment>> {
    if count == 0 {
        return None;
    }

    text(
        format_str,
        config_path,
        context,
        |variable| match variable {
            "count" => Some(count.to_string()),
            _ => None,
        },
    )
}

pub fn symbol(format_str: &str, config_path: &str, context: &Context) -> Option<Vec<Segment>> {
    text(format_str, config_path, context, |_variable| None)
}
