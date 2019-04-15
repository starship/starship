use super::Segment;
use ansi_term::Color;
use clap::ArgMatches;
use std::path::Path;

/// Creates a segment for the prompt character
///
/// The char segment prints an arrow character in a color dependant on the exit-
/// code of the last executed command:
/// - If the exit-code was "0", the arrow will be formatted with `COLOR_SUCCESS`
/// (green by default)
/// - If the exit-code was anything else, the arrow will be formatted with
/// `COLOR_FAILURE` (red by default)
pub fn segment(_current_dir: &Path, args: &ArgMatches) -> Option<Segment> {
    const PROMPT_CHAR: &str = "➜";
    const COLOR_SUCCESS: Color = Color::Green;
    const COLOR_FAILURE: Color = Color::Red;

    let mut segment = Segment::new("char");

    if args.value_of("status_code").unwrap() == "0" {
        segment.set_style(COLOR_SUCCESS);
    } else {
        segment.set_style(COLOR_FAILURE);
    };

    segment.set_value(PROMPT_CHAR).set_prefix(None);

    Some(segment)
}
