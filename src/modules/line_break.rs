use super::Segment;
use clap::ArgMatches;

/// Creates a segment for the line break
pub fn segment(_: &ArgMatches) -> Segment {
    const LINE_ENDING: &str = "\n";

    let mut segment = Segment::new("line_break");

    segment.set_value(LINE_ENDING).clone()
}
