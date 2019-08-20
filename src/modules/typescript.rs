use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a module with the current typescript version
///
/// Will display the typescript version if any of the following criteria are met:
///     - Current directory contains a `.ts` file
///     - Current directory contains a `tsconfig.json` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_ts_project = context
        .new_scan_dir()
        .set_files(&["tsconfig.json"])
        .set_extensions(&["ts"])
        .scan();

    if !is_ts_project {
        return None;
    }

    match get_typescript_version() {
        Some(typescript_version) => {
            const TYPESCRIPT_CHAR: &str = "■ ";
            let module_color = Color::Blue.bold();

            let mut module = context.new_module("typescript")?;
            module.set_style(module_color);

            let formatted_version = format_typescript_version(&typescript_version);
            module.new_segment("symbol", TYPESCRIPT_CHAR);
            module.new_segment("version", &formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn get_typescript_version() -> Option<String> {
    match Command::new("tsc").arg("--version").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

fn format_typescript_version(typescript_version: &str) -> String {
    format!("v{}", typescript_version.trim_start_matches("Version ").trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_typescript_version() {
        let input = "Version 3.5.3";
        assert_eq!(format_typescript_version(input), "v3.5.3");
    }
}