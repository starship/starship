use super::Segment;
use ansi_term::{Color, Style};
use clap::ArgMatches;
use std::env;
use std::fs::{self, DirEntry};
use std::process::Command;

/// Creates a segment with the current Node.js version
/// 
/// Will display the Node.js version if any of the following criteria are met:
///     - Current directory contains a `.js` file
///     - Current directory contains a `node_modules` directory
///     - Current directory contains a `package.json` file
pub fn segment(_: &ArgMatches) -> Segment {
    const NODE_CHAR: &str = "⬢";
    const SECTION_COLOR: Color = Color::Green;

    let current_path = env::current_dir().expect("Unable to identify current directory");
    let files = fs::read_dir(&current_path).unwrap();

    // Early return if there are no JS project files
    let is_js_project = files.filter_map(Result::ok).any(has_js_files);
    if !is_js_project {
        return Segment::default();
    }

    let version = match Command::new("node").arg("--version").output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap().trim().to_string(),
        Err(_) => {
            return Segment::default();
        }
    };

    Segment {
        value: format!("{} {}", NODE_CHAR, version),
        style: Style::from(SECTION_COLOR),
        ..Default::default()
    }
}

fn has_js_files(dir_entry: DirEntry) -> bool {
    let is_js_file = |d: &DirEntry| -> bool {
        d.path().is_file() && d.path().extension().unwrap_or_default() == "js"
    };
    let is_node_modules = |d: &DirEntry| -> bool {
        d.path().is_dir() && d.path().file_name().unwrap_or_default() == "node_modules"
    };
    let is_package_json = |d: &DirEntry| -> bool {
        d.path().is_file() && d.path().file_name().unwrap_or_default() == "package.json"
    };

    is_js_file(&dir_entry) || is_node_modules(&dir_entry) || is_package_json(&dir_entry)
}
