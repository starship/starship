use super::{Segment, PromptComponent};
use crate::context::Context;

/// Creates a segment for the line break
pub fn segment(_context: &Context) -> PromptComponent {
    const LINE_ENDING: &str = "\n";

    let mut segment = Segment::new("line_break");

    segment.set_value(LINE_ENDING);

    Some(Box::new(segment))
}
