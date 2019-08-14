use ansi_term::Color;
use chrono::{DateTime, Local};

use super::{Context, Module};

/// Outputs the current time
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("time")?;

    let is_12hr = module.config_value_bool("12hr").unwrap_or(false);
    let default_format = if is_12hr { "%I:%M:%p" } else { "%H:%M" };
    let time_format = module
        .config_value_str("format")
        .unwrap_or(default_format)
        .to_owned();

    let local: DateTime<Local> = Local::now();

    module.set_style(Color::Yellow.bold());
    module.new_segment("time", &format!("[{}]", local.format(&time_format)));
    module.get_prefix().set_value("");

    Some(module)
}
