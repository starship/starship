use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a module with the current Elixir version
///
/// Will display the Elixir version if any of the following criteria are met:
///     - Current directory contains a `.ex` file
///     - Current directory contains a `mix.exs` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_ex_project = context
        .try_begin_scan()?
        .set_files(&["mix.exs"])
        .set_extensions(&["ex", "exs"])
        .is_match();

    if !is_ex_project {
        return None;
    }

    match get_elixir_version() {
        Some(elixir_version) => {
            const ELIXIR_CHAR: &str = "🧪 ";

            let mut module = context.new_module("elixir");
            let module_style = module
                .config_value_style("style")
                .unwrap_or_else(|| Color::Purple.bold());
            module.set_style(module_style);

            let formatted_version = format_elixir_version(&elixir_version)?;
            module.new_segment("symbol", ELIXIR_CHAR);
            module.new_segment("version", &formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn get_elixir_version() -> Option<String> {
    match Command::new("elixir").arg("-v").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

fn format_elixir_version(version_str: &str) -> Option<String> {
    let version_line = version_str.lines().last()?.trim();
    // ignore detailed Erlang output

    let version = version_line.split_whitespace().nth(1)?;
    // split into ["Elixir", "1.9.1", "(compiled", "with, "Erlang/OTP", "21)"]

    let otp_version = version_line.split_whitespace().nth(5)?.trim_matches(')');

    let mut formatted_version = String::with_capacity(version.len() + otp_version.len() + 6);
    formatted_version.push('v');
    formatted_version.push_str(version);
    formatted_version.push_str(" OTP/");
    formatted_version.push_str(otp_version);
    Some(formatted_version)
}
