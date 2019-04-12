use clap::ArgMatches;
use std::io::{self, Write};

use crate::modules;
use crate::modules::Segment;

pub fn prompt(args: ArgMatches) {
    let default_prompt = vec!["directory", "node", "line_break", "character"];

    // TODO:
    // - List files in directory
    // - Index binaries in PATH

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // Write a new line before the prompt
    writeln!(handle).unwrap();

    default_prompt
        .into_iter()
        .map(|module| modules::handle(module, &args))
        .map(stringify_segment)
        .for_each(|segment_string| write!(handle, "{}", segment_string).unwrap());
}

/// Create a string with the formatted contents of a segment
///
/// Will recursively also format the prefix and suffix of the segment being
/// stringified.
///
/// # Example
/// ```
/// use starship::modules::Segment;
///
/// let segment = Segment {
///     value: String::from("->"),
///     ..Default::default()
/// };
///
/// let result = starship::print::stringify_segment(segment);
/// assert_eq!(result, "-> ");
/// ```
pub fn stringify_segment(segment: Segment) -> String {
    let Segment {
        prefix,
        value,
        style,
        suffix,
    } = segment;

    let mut segment_string = String::new();

    if let Some(prefix) = prefix {
        segment_string += &stringify_segment(*prefix);
    }

    segment_string += &style.paint(value).to_string();

    if let Some(suffix) = suffix {
        segment_string += &stringify_segment(*suffix);
    }

    segment_string
}
