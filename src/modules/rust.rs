use crate::utils::project_detector;
use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a segment with the current Rust version
///
/// Will display the Rust version if any of the following criteria are met:
///     - Current directory contains a file with a `.rs` extension
///     - Current directory contains a `Cargo.toml` file
pub fn segment(context: &Context) -> Option<Module> {
    let rust_criteria = project_detector::Criteria::new()
        .set_files(vec!["Cargo.toml"])
        .set_extensions(vec!["rs"]);

    let is_rs_project = project_detector::is_lang_project(&context.dir_files, &rust_criteria);
    if !is_rs_project {
        return None;
    }

    match get_rust_version() {
        Some(rust_version) => {
            const RUST_CHAR: &str = "🦀 ";
            let module_color = Color::Red.bold();

            let mut module = Module::new("rust");
            module.set_style(module_color);

            let formatted_version = format_rustc_version(rust_version);
            module.new_segment("symbol", RUST_CHAR);
            module.new_segment("version", formatted_version);

            Some(module)
        }
        None => None,
    }
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
