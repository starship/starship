use ansi_term::Color;

use super::{Context, Module};

/// Creates a segment to show if there are any active jobs running
pub fn segment<'a>(context: &'a Context) -> Option<Module<'a>> {
    const JOB_CHAR: &str = "✦ ";
    let module_color = Color::Blue.bold();

    let mut module = context.new_module("jobs");
    module.set_style(module_color);

    let arguments = &context.arguments;
    let num_of_jobs = arguments.value_of("jobs").unwrap_or("0");
    if num_of_jobs == "0" {
        return None;
    } else {
        module.new_segment("symbol", PROMPT_CHAR);
        if num_of_jobs != "1" {
            module.new_segment("number", num_of_jobs);
        }
    };

    Some(module)
}
