use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a module with the current Crystal version
///
/// Will display the Crystal version if any of the following criteria are met:
///     - Current directory contains a `.cr` file
///     - Current directory contains a `shard.yml` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_crystal_project = context
        .new_scan_dir()
        .set_files(&["shard.yml"])
        .set_extensions(&["cr"])
        .scan();

    if !is_crystal_project {
        return None;
    }

    match get_crystal_version() {
        Some(crystal_version) => {
            const CRYSTAL_CHAR: &str = "🔮 ";
            let module_color = Color::Red.bold();

            let mut module = context.new_module("crystal")?;
            module.set_style(module_color);

            let formatted_version = format_crystal_version(&crystal_version)?;
            module.new_segment("symbol", CRYSTAL_CHAR);
            module.new_segment("version", &formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn get_crystal_version() -> Option<String> {
    match Command::new("crystal").arg("-v").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

fn format_crystal_version(crystal_version: &str) -> Option<String> {
    let version = crystal_version
        // split into ["Crystal", "0.30.1", ...]
        .split_whitespace()
        // return "2.6.0p0"
        .nth(1)?;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}
