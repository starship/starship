use std::process::Command;

use super::{Context, Module};

use crate::config::RootModuleConfig;
use crate::configs::nim::NimConfig;

/// Creates a module with the current Nim environment
///
/// Will display the Nim environment if `*.nimble` is found in the directory.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_nim_project = context
        .try_begin_scan()?
        .set_extensions(&["nimble"])
        .is_match();

    if !is_nim_project {
        return None;
    }

    match get_nim_version() {
        Some(raw_nim_version) => {
            let nim_version = format_nim_version(&raw_nim_version)?;

            let mut module = context.new_module("nim");
            let config = NimConfig::try_load(module.config);

            module.set_style(config.style);

            module.create_segment("symbol", &config.symbol);
            module.create_segment("environment", &config.environment.with_value(&nim_version));

            Some(module)
        }
        None => None,
    }
}

fn get_nim_version() -> Option<String> {
    match Command::new("nim").arg("--version").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

fn format_nim_version(nim_version: &str) -> Option<String> {
    let version = nim_version
        // split into ["Nim", "Compiler", "Version", "0.19.4", "[Linux:", "amd64]", ...]
        .split_whitespace()
        // return "0.19.4"
        .nth(3)?;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_nim_version() {
        let nim_output = "Nim Compiler Version 0.19.4 [Linux: amd64]
Compiled at 2019-03-05
Copyright (c) 2006-2018 by Andreas Rumpf

active boot switches: -d:release";

        let actual = format_nim_version(nim_output).unwrap();
        assert_eq!(actual, "v0.19.4");
    }

    #[test]
    fn test_format_nim_version_with_short_version() {
        let nim_output = "Nim Compiler Version 1.2.3 [Linux: amd64]
Compiled at 2019-03-05
Copyright (c) 2006-2018 by Andreas Rumpf

active boot switches: -d:release";

        let actual = format_nim_version(nim_output).unwrap();
        assert_eq!(actual, "v1.2.3");
    }

    #[test]
    fn test_format_nim_version_with_long_version() {
        let nim_output = "Nim Compiler Version 999.888.777 [Linux: amd64]
Compiled at 2019-03-05
Copyright (c) 2006-2018 by Andreas Rumpf

active boot switches: -d:release";

        let actual = format_nim_version(nim_output).unwrap();
        assert_eq!(actual, "v999.888.777");
    }
}
