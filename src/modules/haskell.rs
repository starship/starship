use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a module with the current Haskell Stack version
///
/// Will display the Haskell version if any of the following criteria are met:
///     - Current directory contains a `stack.yaml` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_haskell_project = context
        .try_begin_scan()?
        .set_files(&["stack.yaml"])
        .is_match();

    if !is_haskell_project {
        return None;
    }

    match get_haskell_version() {
        Some(haskell_version) => {
            const HASKELL_CHAR: &str = "λ ";

            let mut module = context.new_module("haskell");
            let module_style = module
                .config_value_style("style")
                .unwrap_or_else(|| Color::Red.bold());
            module.set_style(module_style);

            let formatted_version = format_haskell_version(&haskell_version)?;
            module.new_segment("symbol", HASKELL_CHAR);
            module.new_segment("version", &formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn get_haskell_version() -> Option<String> {
    match Command::new("stack")
        .arg("ghc")
        .arg("--")
        .arg("--numeric-version")
        .arg("--no-install-ghc")
        .output()
    {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

fn format_haskell_version(haskell_version: &str) -> Option<String> {
    let version = haskell_version;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}
