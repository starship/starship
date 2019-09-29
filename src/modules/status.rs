use ansi_term::Color;

use super::{Context, Module};

#[derive(PartialEq)]
enum DisplayMode {
    Always,      // Always shows all exit codes
    OnExitError, // Show all exit codes if there's any error in the pipeline
    OnError,     // Show all exit codes only on error
    OnErrorOrMismatch, // Show exit codes when pipeline and exit code don't match
                 //     or there's an error in the pipeline
}

/// Creates a module for exit codes from the previous pipeline
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("status");
    module.get_prefix().set_value("");
    let arguments = &context.arguments;
    let exit_code = arguments.value_of("status_code")?;
    let pipestatus: Vec<&str> = match arguments.value_of("pipestatus") {
        Some(val) => val.split_ascii_whitespace().collect(),
        // fallback if --pipestatus is not provided
        None => vec![&exit_code],
    };

    // kind of a hack to not show `<previous status code>` when a user sends ^C to clear the line
    if exit_code == "130" && *pipestatus.last()? != "130" {
        module.get_suffix().set_value("");
        return Some(module);
    }

    let error = exit_code != "0";
    let pipeline_error = pipestatus.iter().any(|&code| code != "0");
    let mismatch = exit_code != *pipestatus.last()?;

    let display_mode = get_display_mode(&module);
    let show = match display_mode {
        DisplayMode::Always => true,
        DisplayMode::OnExitError => error,
        DisplayMode::OnError => pipeline_error,
        DisplayMode::OnErrorOrMismatch => pipeline_error || mismatch,
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
    module.set_style(if error { error_style } else { success_style });

    let symbols = module.config_value_bool("use_symbols").unwrap_or(false);
    // It's probably not the cleanest solution but, according to my testing,
    // it doesn't affect the performance.
    // And I don't know how to make it work without making the borrow checker complain :^)
    // If people won't use this feature we could just remove it altogether
    let (pipe_output, exit_output) = if symbols {
        const SUCCESS_CHAR: &str = "✔";
        const ERROR_CHAR: &str = "✖";
        let success_symbol = module
            .config_value_str("success_symbol")
            .unwrap_or(SUCCESS_CHAR);
        let error_symbol = module
            .config_value_str("error_symbol")
            .unwrap_or(ERROR_CHAR);
        let pipe_output = pipestatus
            .into_iter()
            .map(|code| {
                if code == "0" {
                    success_symbol
                } else {
                    error_symbol
                }
            })
            .collect();
        let exit_output = if exit_code == "0" {
            success_symbol.to_string()
        } else {
            error_symbol.to_string()
        };
        (pipe_output, exit_output)
    } else {
        (pipestatus, exit_code.to_string())
    };

    let simple_pipeline = module.config_value_bool("simple_pipeline").unwrap_or(true);
    let no_pipeline = (simple_pipeline && !pipeline_error) || pipe_output.len() == 1;
    let output = if no_pipeline {
        pipe_output.last()?.to_string()
    } else {
        const PIPE_PREFFIX: &str = "(";
        const PIPE_SUFFIX: &str = ")";
        let prefix = module
            .config_value_str("pipe_prefix")
            .unwrap_or(PIPE_PREFFIX);
        let suffix = module
            .config_value_str("pipe_suffix")
            .unwrap_or(PIPE_SUFFIX);
        format!("{}{}{}", prefix, pipe_output.join(" "), suffix)
    };

    module.new_segment("pipe", &output);

    if mismatch {
        const EXIT_PREFIX: &str = "[";
        const EXIT_SUFFIX: &str = "]";
        let prefix = module
            .config_value_str("exit_prefix")
            .unwrap_or(EXIT_PREFIX);
        let suffix = module
            .config_value_str("exit_suffix")
            .unwrap_or(EXIT_SUFFIX);
        let code = format!(" {}{}{}", prefix, exit_output, suffix);
        module.new_segment("exit", &code);
    }

    Some(module)
}

fn get_display_mode(module: &Module) -> DisplayMode {
    match module.config_value_str("display_mode") {
        Some("always") => DisplayMode::Always,
        Some("error") => DisplayMode::OnExitError,
        Some("any error") => DisplayMode::OnError,
        /*Some("mismatch") |*/ _ => DisplayMode::OnErrorOrMismatch,
    }
}
