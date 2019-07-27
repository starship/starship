use ansi_term::Color;
use std::process::Command;

use super::{Context, Module};

/// Creates a module with the current Node.js version
///
/// Will display the Node.js version if any of the following criteria are met:
///     - Current directory contains a `.js` file
///     - Current directory contains a `package.json` file
///     - Current directory contains a `node_modules` directory
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_js_project = context
        .new_scan_dir()
        .set_files(&["package.json"])
        .set_extensions(&["js"])
        .set_folders(&["node_modules"])
        .scan();

    if !is_js_project {
        return None;
    }

    match get_node_version() {
        Some(node_version) => {
            const NODE_CHAR: &str = "⬢ ";
            let module_color = Color::Green.bold();

            let mut module = context.new_module("node")?;
            module.set_style(module_color);

            let formatted_version = node_version.trim();
            module.new_segment("symbol", NODE_CHAR);
            module.new_segment("version", formatted_version);

            Some(module)
        }
        None => None,
    }
}

fn get_node_version() -> Option<String> {
    match Command::new("node").arg("--version").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}
