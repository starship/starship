use super::Segment;
use clap::ArgMatches;

/// Creates a segment for the line break
pub fn segment(_: &ArgMatches) -> Option<Segment> {
    const LINE_ENDING: &str = "\n";

    let mut segment = Segment::new("line_break");

    segment
        .set_value(LINE_ENDING)
        .set_prefix(None)
        .set_suffix(None);

    Some(segment)
}
