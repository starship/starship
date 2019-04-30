use ansi_term::Color;
use std::path::PathBuf;
use std::process::Command;

use super::{Context, Module};

/// Creates a segment with the current Rust version
///
/// Will display the Rust version if any of the following criteria are met:
///     - Current directory contains a `.rs` file
///     - Current directory contains a `Cargo.toml` file
pub fn segment(context: &Context) -> Option<Module> {
    let is_rs_project = context.dir_files.iter().any(has_rs_files);
    if !is_rs_project {
        return None;
    }

    match get_rust_version() {
        Some(rust_version) => {
            const RUST_CHAR: &str = "🦀";
            const MODULE_COLOR: Color = Color::Red;

            let mut module = Module::new("rust");
            module.set_style(MODULE_COLOR.bold());

            let symbol = module.new_segment("symbol");
            symbol.set_value(RUST_CHAR);

            let version = module.new_segment("version");
            let formatted_version = format_rustc_version(rust_version);
            version.set_value(formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn has_rs_files(dir_entry: &PathBuf) -> bool {
    let is_rs_file =
        |d: &PathBuf| -> bool { d.is_file() && d.extension().unwrap_or_default() == "rs" };
    let is_cargo_toml =
        |d: &PathBuf| -> bool { d.is_file() && d.file_name().unwrap_or_default() == "Cargo.toml" };

    is_rs_file(&dir_entry) || is_cargo_toml(&dir_entry)
}

fn get_rust_version() -> Option<String> {
    match Command::new("rustc").arg("-V").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

fn format_rustc_version(mut rustc_stdout: String) -> String {
    let offset = &rustc_stdout.find('(').unwrap();
    let formatted_version: String = rustc_stdout.drain(..offset).collect();

    format!("v{}", formatted_version.replace("rustc", "").trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_rustc_version() {
        let nightly_input = String::from("rustc 1.34.0-nightly (b139669f3 2019-04-10)");
        assert_eq!(format_rustc_version(nightly_input), "v1.34.0-nightly");

        let beta_input = String::from("rustc 1.34.0-beta.1 (2bc1d406d 2019-04-10)");
        assert_eq!(format_rustc_version(beta_input), "v1.34.0-beta.1");

        let stable_input = String::from("rustc 1.34.0 (91856ed52 2019-04-10)");
        assert_eq!(format_rustc_version(stable_input), "v1.34.0");
    }
}
