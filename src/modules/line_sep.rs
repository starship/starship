use super::Segment;
use clap::ArgMatches;

/// Creates a segment for the line break
pub fn segment(_: &ArgMatches) -> Segment {
    const LINE_ENDING: &str = "\n";

    Segment {
        value: String::from(LINE_ENDING),
        suffix: None,
        ..Default::default()
    }
}
