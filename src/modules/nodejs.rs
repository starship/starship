use ansi_term::Color;
use std::path::PathBuf;
use std::process::Command;

use super::Segment;
use crate::context::Context;

/// Creates a segment with the current Node.js version
///
/// Will display the Node.js version if any of the following criteria are met:
///     - Current directory contains a `.js` file
///     - Current directory contains a `package.json` file
///     - Current directory contains a `node_modules` directory
pub fn segment(context: &Context) -> Option<Segment> {
    let is_js_project = context.dir_files.iter().any(has_js_files);
    if !is_js_project {
        return None;
    }

    match get_node_version() {
        Some(node_version) => {
            const NODE_CHAR: &str = "⬢";
            const SEGMENT_COLOR: Color = Color::Green;

            let mut segment = Segment::new("node");
            segment.set_style(SEGMENT_COLOR.bold());

            let formatted_version = node_version.trim();
            segment.set_value(format!("{} {}", NODE_CHAR, formatted_version));

            Some(segment)
        }
        None => None,
    }
}

fn has_js_files(dir_entry: &PathBuf) -> bool {
    let is_js_file =
        |d: &PathBuf| -> bool { d.is_file() && d.extension().unwrap_or_default() == "js" };
    let is_node_modules =
        |d: &PathBuf| -> bool { d.is_dir() && d.file_name().unwrap_or_default() == "node_modules" };
    let is_package_json = |d: &PathBuf| -> bool {
        d.is_file() && d.file_name().unwrap_or_default() == "package.json"
    };

    is_js_file(&dir_entry) || is_node_modules(&dir_entry) || is_package_json(&dir_entry)
}

fn get_node_version() -> Option<String> {
    match Command::new("node").arg("--version").output() {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}
