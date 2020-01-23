use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::crystal::CrystalConfig;
use crate::utils;

/// Creates a module with the current Crystal version
///
/// Will display the Crystal version if any of the following criteria are met:
///     - Current directory contains a `.cr` file
///     - Current directory contains a `shard.yml` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_crystal_project = context
        .try_begin_scan()?
        .set_files(&["shard.yml"])
        .set_extensions(&["cr"])
        .is_match();

    if !is_crystal_project {
        return None;
    }

    let crystal_version = utils::exec_cmd("crystal", &["-v"])?.stdout;
    let formatted_version = format_crystal_version(&crystal_version)?;

    let mut module = context.new_module("crystal");
    let config: CrystalConfig = CrystalConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(&formatted_version));

    Some(module)
}

fn format_crystal_version(crystal_version: &str) -> Option<String> {
    let version = crystal_version
        // split into ["Crystal", "0.32.1", ...]
        .split_whitespace()
        // return "0.32.1"
        .nth(1)?;

    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
}
