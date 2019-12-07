use super::utils::query_parser::*;
use super::{Context, Module, RootModuleConfig};

use crate::configs::nodejs::NodejsConfig;
use crate::segment::Segment;
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

    let formatted_version = node_version.trim();

    let segments: Vec<Segment> = format_segments(config.format, None, |name, query| {
        let style = get_style_from_query(&query);
        match name {
            "version" => Some(Segment {
                _name: "version".to_string(),
                value: formatted_version.to_string(),
                style,
            }),
            _ => None,
        }
    })
    .ok()?;

    module.set_segments(segments);

    Some(module)
}
