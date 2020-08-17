use super::{Context, Module};
use crate::segment::Segment;

/// Creates a module for the line break
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const LINE_ENDING: &str = "\n";

    let mut module = context.new_module("line_break");

    module.set_segments(vec![Segment::new(None, LINE_ENDING)]);

    Some(module)
}
