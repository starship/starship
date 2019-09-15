use ansi_term::Color;
use_command!();

use super::{Context, Module};

/// Creates a module with the current Ruby version
///
/// Will display the Ruby version if any of the following criteria are met:
///     - Current directory contains a `.rb` file
///     - Current directory contains a `Gemfile` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_rb_project = context
        .try_begin_scan()?
        .set_files(&["Gemfile"])
        .set_extensions(&["rb"])
        .is_match();

    if !is_rb_project {
        return None;
    }

    match get_ruby_version() {
        Some(ruby_version) => {
            const RUBY_CHAR: &str = "💎 ";

            let mut module = context.new_module("ruby");
            let module_style = module
                .config_value_style("style")
                .unwrap_or_else(|| Color::Red.bold());
            module.set_style(module_style);

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
        .nth(1)?
        .get(0..5)?;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}
