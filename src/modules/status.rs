use super::{Context, Module};

use crate::config::RootModuleConfig;
use crate::configs::status::{DisplayMode, StatusConfig};

/// Creates a module for exit codes from the previous pipeline
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("status");
    let status_config = StatusConfig::try_load(module.config);
    let props = &context.properties;
    let mut exit_code = props.get("status_code")?.as_ref();
    let mut pipestatus: Vec<&str> = match props.get("pipestatus") {
        Some(val) => val.split_ascii_whitespace().collect(),
        // fallback if --pipestatus is not provided
        None => vec![&exit_code],
    };

    // kind of a hack to not show `<previous status code>` when a user sends ^C to clear the line
    if exit_code == "130" && *pipestatus.last()? != "130" {
        return None;
    }

    let error = exit_code != "0";
    let pipeline_error = pipestatus.iter().any(|&code| code != "0");
    let mismatch = exit_code != *pipestatus.last()?;

    let display_mode = status_config.display_mode; //get_display_mode(&module);
    let show = match display_mode {
        DisplayMode::Always => true,
        DisplayMode::OnExitError => error,
        DisplayMode::OnError => pipeline_error,
        DisplayMode::OnErrorOrMismatch => pipeline_error || mismatch,
    };
    if !show {
        return None;
    }

    module.get_prefix().set_value("");

    let success_style = status_config.success;
    let error_style = status_config.error;
    module.set_style(if error { error_style } else { success_style });

    let symbols = status_config.use_symbols;
    if symbols {
        let success_symbol = status_config.success_symbol;
        let error_symbol = status_config.error_symbol;
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
        exit_code = if exit_code == "0" {
            success_symbol
        } else {
            error_symbol
        };
    }

    let simple_pipeline = status_config.simple_pipeline;
    let no_pipeline = pipestatus.len() == 1
        || (simple_pipeline && (!pipeline_error || no_repetitions(&pipestatus)));
    let output = if no_pipeline {
        pipestatus.last()?.to_string()
    } else {
        let prefix = status_config.pipe_prefix;
        let suffix = status_config.pipe_suffix;
        format!("{}{}{}", prefix, pipestatus.join(" "), suffix)
    };

    module.create_segment("pipeline", &status_config.pipeline.with_value(&output));

    if mismatch {
        let prefix = status_config.exit_prefix;
        let suffix = status_config.exit_suffix;
        let code = format!(" {}{}{}", prefix, exit_code, suffix);
        module.create_segment("exitcode", &status_config.exitcode.with_value(&code));
    }

    Some(module)
}

fn no_repetitions<'a, 'b: 'a>(pipeline: &'b [&'a str]) -> bool {
    for status in pipeline[1..].iter() {
        if *status != pipeline[0] {
            return false;
        }
    }
    true
}
