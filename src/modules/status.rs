use ansi_term::Color;

use super::{Context, Module};

#[derive(PartialEq)]
enum DisplayMode {
    Always,          // Always shows all exit codes
    OnError,         // Show all exit codes only on error
    PipelineOrError, // Always shows if there's more than one exit code, otherwise only on error
    Last,            // Always shows the last exit code
    LastOnError,     // Only shows the last exit code and only on error
}

/// Creates a module for exit codes from the previous pipeline
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("status");
    module.get_prefix().set_value("");
    let arguments = &context.arguments;
    let exit_code = arguments.value_of("status_code")?;
    let mut pipestatus: Vec<&str> = match arguments.value_of("pipestatus") {
        Some(val) => val.split_ascii_whitespace().collect(),
        // fallback if --pipestatus is not provided
        None => vec![exit_code],
    };

    // kind of a hack to not show `!<previous status code>` when a user sends ^C to clear the line
    if exit_code == "130" && *pipestatus.last()? != "130" {
        module.get_suffix().set_value("");
        return Some(module);
    }

    let errored = exit_code != "0";

    let display_mode = get_display_mode(&module);
    let show = match display_mode {
        DisplayMode::Always | DisplayMode::Last => true,
        DisplayMode::PipelineOrError => errored || pipestatus.len() > 1,
        DisplayMode::OnError | DisplayMode::LastOnError => errored,
    };
    if !show {
        module.get_suffix().set_value("");
        return Some(module);
    }

    let success_style = module
        .config_value_style("success")
        .unwrap_or_else(|| Color::White.dimmed());
    let error_style = module
        .config_value_style("error")
        .unwrap_or_else(|| Color::Red.into());
    match exit_code {
        "0" => module.set_style(success_style),
        _ => module.set_style(error_style),
    };

    if exit_code != *pipestatus.last()? {
        module.new_segment("negation", "!").set_style(if !errored {
            success_style
        } else {
            error_style
        });
    }

    let symbols = module.config_value_bool("use_symbols").unwrap_or(false);
    if symbols {
        let success_symbol = module.config_value_str("success_symbol").unwrap_or("✔");
        let error_symbol = module.config_value_str("error_symbol").unwrap_or("✖");
        pipestatus = pipestatus
            .into_iter()
            .map(|code| {
                if code == "0" {
                    success_symbol
                } else {
                    error_symbol
                }
            })
            .collect();
    }

    let no_pipeline = pipestatus.len() == 1
        || display_mode == DisplayMode::Last
        || display_mode == DisplayMode::LastOnError;
    let output = if no_pipeline {
        pipestatus.last()?.to_string()
    } else {
        let prefix = module.config_value_str("prefix").unwrap_or("(");
        let suffix = module.config_value_str("suffix").unwrap_or(")");
        format!("{}{}{}", prefix, pipestatus.join(" "), suffix)
    };

    module.new_segment("status", &output);

    Some(module)
}

fn get_display_mode(module: &Module) -> DisplayMode {
    match module.config_value_str("display_mode") {
        Some("always") => DisplayMode::Always,
        Some("error") => DisplayMode::OnError,
        Some("last") => DisplayMode::Last,
        Some("last on error") => DisplayMode::LastOnError,
        /*Some("pipeline or error") |*/ _ => DisplayMode::PipelineOrError,
    }
}
