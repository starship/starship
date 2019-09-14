use ansi_term::Color;

use super::{Context, Module};

/// Creates a module for the line break
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("status");
    module.get_prefix().set_value("");
    let mut pipestatus: Vec<&str> = context
        .arguments
        .value_of("status_code")?
        .split_ascii_whitespace()
        .collect();

    let exit_code = *pipestatus.last()?;

    let show_success = module.config_value_bool("show_success").unwrap_or(false);
    let show_pipeline_always = module
        .config_value_bool("show_pipeline_always")
        .unwrap_or(true);
    let no_pipeline = module.config_value_bool("no_pipeline").unwrap_or(false);
    let not_show = (!show_success && exit_code == "0")
        && (no_pipeline || !show_pipeline_always || pipestatus.len() == 1);
    if not_show {
        module.get_suffix().set_value("");
        return Some(module);
    }

    let success_style = module
        .config_value_style("success")
        .unwrap_or(Color::White.dimmed());
    let error_style = module
        .config_value_style("error")
        .unwrap_or(Color::Red.into());
    match exit_code {
        "0" => module.set_style(success_style),
        _ => module.set_style(error_style),
    };

    let icons = module.config_value_bool("icons").unwrap_or(false);
    if icons {
        let success_icon = module.config_value_str("success_symbol").unwrap_or("✔");
        let error_icon = module.config_value_str("error_symbol").unwrap_or("✘");
        pipestatus = pipestatus
            .into_iter()
            .map(|code| {
                if code == "0" {
                    success_icon
                } else {
                    error_icon
                }
            })
            .collect();
    }

    let output = if !no_pipeline {
        match pipestatus.len() {
            1 => pipestatus[0].to_string(),
            _ => format!("({})", pipestatus.join(" ")),
        }
    } else {
        exit_code.to_string()
    };

    module.new_segment("status", &output);

    Some(module)
}
