use super::{Context, Module};
use crate::config::SegmentConfig;

/// Creates a module for the line break
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const LINE_ENDING: &str = "\n";

    let mut module = context.new_module("line_break");

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    module.create_segment(
        "character",
        &SegmentConfig {
            value: LINE_ENDING,
            style: None,
        },
    );

    Some(module)
}
