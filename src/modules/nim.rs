use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a module with the current Nim version
///
/// Will display the Nim version if any of the following criteria are met:
///     - Current directory contains a `.nim` file
///     - Current directory contains a `.nimble` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_nim_project = context
        .new_scan_dir()
        .set_extensions(&["nim", "nimble"])
        .scan();

    if !is_nim_project {
        return None;
    }

    match get_nim_version() {
        Some(nim_version) => {
            const NIM_CHAR: &str = "👑 ";
            let module_color = Color::Red.bold();

            let mut module = context.new_module("nim")?;
            module.set_style(module_color);

            let formatted_version = format_nim_version(&nim_version)?;
            module.new_segment("symbol", NIM_CHAR);
            module.new_segment("version", &formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn get_nim_version() -> Option<String> {
    match Command::new("nim").arg("-v").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

fn format_nim_version(nim_version: &str) -> Option<String> {
    let version = nim_version
        // split into ["Nim", "Compiler", "Version", "0.20.2", "[Linux: amd64]"]
        .split_whitespace()
        // return "0.20.2"
        .nth(3)?;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}
