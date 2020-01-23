use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::nodejs::NodejsConfig;
use crate::utils;

/// Creates a module with the current Node.js version
///
/// Will display the Node.js version if any of the following criteria are met:
///     - Current directory contains a `.js` file
///     - Current directory contains a `package.json` file
///     - Current directory contains a `node_modules` directory
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_js_project = context
        .try_begin_scan()?
        .set_files(&["package.json"])
        .set_extensions(&["js"])
        .set_folders(&["node_modules"])
        .is_match();

    if !is_js_project {
        return None;
    }

    let node_version = utils::exec_cmd("node", &["--version"])?.stdout;

    let mut module = context.new_module("nodejs");
    let config: NodejsConfig = NodejsConfig::try_load(module.config);

    module.set_style(config.style);

    let formatted_version = node_version.trim();
    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(formatted_version));

    Some(module)
}
