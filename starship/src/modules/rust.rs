use std::process::Command;

use super::{Context, Module};

use crate::config::RootModuleConfig;
use crate::configs::rust::RustConfig;

/// Creates a module with the current Rust version
///
/// Will display the Rust version if any of the following criteria are met:
///     - Current directory contains a file with a `.rs` extension
///     - Current directory contains a `Cargo.toml` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_rs_project = context
        .try_begin_scan()?
        .set_files(&["Cargo.toml"])
        .set_extensions(&["rs"])
        .is_match();

    if !is_rs_project {
        return None;
    }
    let config = RustConfig::try_load(context.config.config.as_ref());

    match get_rust_version() {
        Some(rust_version) => {
            let mut module = context.new_module("rust");
            module.set_style(config.style);

            let formatted_version = format_rustc_version(rust_version);
            module.new_segment("symbol", config.symbol);
            module.new_segment("version", &formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn get_rust_version() -> Option<String> {
    match Command::new("rustc").arg("--version").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

fn format_rustc_version(mut rustc_stdout: String) -> String {
    let offset = &rustc_stdout.find('(').unwrap_or_else(|| rustc_stdout.len());
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

        let version_without_hash = String::from("rustc 1.34.0");
        assert_eq!(format_rustc_version(version_without_hash), "v1.34.0");
    }
}
