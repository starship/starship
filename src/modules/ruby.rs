use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a module with the current Ruby version
///
/// Will display the Ruby version if any of the following criteria are met:
///     - Current directory contains a `.rb` file
///     - Current directory contains a `Gemfile` file
///     - Current directory contains a `.ruby-version` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_rb_project = context
        .new_scan_dir()
        .set_files(&["Gemfile", ".ruby-version"])
        .set_extensions(&["rb"])
        .scan();

    if !is_rb_project {
        return None;
    }

    match get_ruby_version() {
        Some(ruby_version) => {
            const RUBY_CHAR: &str = "💎 ";
            let module_color = Color::Red.bold();

            let mut module = context.new_module("ruby")?;
            module.set_style(module_color);

            let formatted_version = format_ruby_version(&ruby_version)?;
            module.new_segment("symbol", RUBY_CHAR);
            module.new_segment("version", &formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn get_ruby_version() -> Option<String> {
    match Command::new("ruby").arg("-v").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

fn format_ruby_version(ruby_version: &str) -> Option<String> {
    let version = ruby_version
        // split into ["ruby", "2.6.0p0", "linux/amd64"]
        .split_whitespace()
        // return "2.6.0p0"
        .nth(1)?;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}
