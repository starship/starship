use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::haskell::HaskellConfig;
use crate::utils;

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

    let haskell_version = utils::exec_cmd(
        "stack",
        &["ghc", "--", "--numeric-version", "--no-install-ghc"],
    )?
    .stdout;
    let formatted_version = format_haskell_version(&haskell_version)?;

    let mut module = context.new_module("haskell");
    let config: HaskellConfig = HaskellConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(&formatted_version));

    Some(module)
}

fn format_haskell_version(haskell_version: &str) -> Option<String> {
    let formatted_version = format!("v{}", haskell_version.trim());
    Some(formatted_version)
}
