use super::{Context, Module};
use crate::segment::Segment;

/// Creates a module for the line break
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const LINE_ENDING: &str = "\n";

    let show_newline = context.config.get_root_config().add_newline;
    if show_newline == false {
        return None;
    }

    let mut module = context.new_module("line_break");

    module.set_segments(vec![Segment {
        _name: "line_break".to_string(),
        style: None,
        value: LINE_ENDING.to_string(),
    }]);

    Some(module)
}
