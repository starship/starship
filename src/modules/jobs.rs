use ansi_term::Color;

use super::{Context, Module};

/// Creates a segment to show if there are any active jobs running
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    const THRESHOLD_DEFAULT: i64 = 1;
    let mut module = context.new_module("jobs")?;

    let threshold = module
        .config_value_i64("threshold")
        .unwrap_or(THRESHOLD_DEFAULT);

    const JOB_CHAR: &str = "✦ ";
    let module_color = Color::Blue.bold();

    module.set_style(module_color);

    let arguments = &context.arguments;
    let num_of_jobs = arguments.value_of("jobs").unwrap_or("0");
    let num_of_jobs: i64 = num_of_jobs.parse().unwrap();
    print(&num_of_jobs.to_string())
    if num_of_jobs == 0 {
        return None;
    }
    module.new_segment("symbol", JOB_CHAR);
    if num_of_jobs > threshold {
        module.new_segment("number", &num_of_jobs.to_string());
    }

    Some(module)
}
