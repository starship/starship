use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::purescript::PureScriptConfig;
use crate::utils;

/// Creates a module with the current PureScript version
///
/// Will display the PureScript version if any of the following criteria are met:
///     - Current directory contains a `spago.dhall` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_purs_project = context
        .try_begin_scan()?
        .set_files(&["spago.dhall"])
        .set_extensions(&["purs"])
        .is_match();

    if !is_purs_project {
        return None;
    }

    let purs_version = utils::exec_cmd("purs", &["--version"])?.stdout;
    let formatted_version = Some(format!("v{}", purs_version.trim()))?;

    let mut module = context.new_module("purescript");
    let config: PureScriptConfig = PureScriptConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(&formatted_version));

    Some(module)
}
