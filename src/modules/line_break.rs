use super::{Context, Module};
use crate::segment::Segment;

/// Creates a module for the line break
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const LINE_ENDING: &str = "\n";

    let mut module = context.new_module("line_break");

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    module.set_segments(vec![Segment {
        _name: "line_break".to_string(),
        style: None,
        value: LINE_ENDING.to_string(),
    }]);

    Some(module)
}
