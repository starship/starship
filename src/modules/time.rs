use ansi_term::Color;
use chrono::{DateTime, Local};

use super::{Context, Module};

/// Outputs the current time
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("time")?;

    let prefix = module.config_value_str("prefix").unwrap_or("").to_owned();
    let suffix = module.config_value_str("suffix").unwrap_or("").to_owned();

    // Remove when logic for disabled by default exists
    if module.config_value_bool("disabled").unwrap_or(true) {
        return None;
    }

    let is_12hr = module.config_value_bool("12hr").unwrap_or(false);
    let default_format = if is_12hr { "%r" } else { "%T" };
    let time_format = module
        .config_value_str("format")
        .unwrap_or(default_format)
        .to_owned();

    let local: DateTime<Local> = Local::now();

    module.set_style(Color::Yellow.bold());
    module.new_segment(
        "time",
        &format!("{}{}{}", prefix, local.format(&time_format), suffix),
    );
    module.get_prefix().set_value("at ");

    Some(module)
}
