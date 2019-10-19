use std::process::Command;

use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::nodejs_yarn::NodejsYarnConfig;

/// Creates a module with the current yarn version
///
/// Will display the yarn version if any of the following criteria are met:
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

    let mut module = context.new_module("nodejs_yarn");
    let config = NodejsYarnConfig::try_load(module.config);

    // This is currently needed for modules that are disabled by default.
    if config.disabled {
        return None;
    }

    match get_yarn_version() {
        Some(yarn_version) => {
            module.get_prefix().set_value("with ");
            module.set_style(config.style);

            let formatted_version = format!("v{}", yarn_version.trim());
            module.create_segment("symbol", &config.symbol);
            module.create_segment("version", &SegmentConfig::new(&formatted_version));

            Some(module)
        }
        None => None,
    }
}

fn get_yarn_version() -> Option<String> {
    match Command::new("yarn").arg("--version").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}
