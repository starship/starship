use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a segment to show if there are any active jobs running
pub fn segment<'a>(context: &'a Context) -> Option<Module<'a>> {
    match get_jobs() {
        Some(is_job) => {
            print!("{}", is_job);

            const JOB_CHAR: &str = "✦ ";
            let module_color = Color::Blue.bold();

            let mut module = context.new_module("jobs");
            module.set_style(module_color);

            if is_job == "0" {
                module.new_segment("symbol", "");
            } else {
                module.new_segment("symbol", JOB_CHAR);
            }

            Some(module)
        }
        None => None,
    }
}

fn get_jobs() -> Option<String> {
    match Command::new("$(jobs -d | awk '!/pwd/' | wc -l | tr -d ' ')").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}
