use std::process::Command;

use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::haskell::HaskellConfig;

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
        None
    } else {
        get_haskell_version().map(|haskell_version| {
            let formatted_version = format_haskell_version(&haskell_version).unwrap();

            let mut module = context.new_module("haskell");
            let config: HaskellConfig = HaskellConfig::try_load(module.config);
            module.set_style(config.style);

            module.create_segment("symbol", &config.symbol);
            module.create_segment("version", &SegmentConfig::new(&formatted_version));
            module
        })
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
    let formatted_version = format!("v {}", haskell_version);
    Some(formatted_version)
}
