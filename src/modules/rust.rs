use super::Segment;
use ansi_term::Color;
use std::fs::{self, DirEntry};
use crate::context::Context;
use std::process::Command;

/// Creates a segment with the current Node.js version
///
/// Will display the Node.js version if any of the following criteria are met:
///     - Current directory contains a `.js` file
///     - Current directory contains a `node_modules` directory
///     - Current directory contains a `package.json` file
pub fn segment(context: &Context) -> Option<Segment> {
    const RUST_LOGO: &str = "🦀";
    const SECTION_COLOR: Color = Color::Red;

    let mut segment = Segment::new("node");
    let files = fs::read_dir(&context.current_dir).unwrap();

    // Early return if there are no JS project files
    let is_rs_project = files.filter_map(Result::ok).any(has_rs_files);
    if !is_rs_project {
        return None;
    }

    match Command::new("rustc").arg("-V").output() {
        Ok(output) => {
            let version = String::from_utf8(output.stdout).unwrap();
            segment.set_value(format!("{}  {}", RUST_LOGO, version.trim()))
        }
        Err(_) => {
            return None;
        }
    };

    segment.set_style(SECTION_COLOR);
    Some(segment)
}

fn has_rs_files(dir_entry: DirEntry) -> bool {
    let is_rs_file = |d: &DirEntry| -> bool {
        d.path().is_file() && d.path().extension().unwrap_or_default() == "rs"
    };
    let is_package_json = |d: &DirEntry| -> bool {
        d.path().is_file() && d.path().file_name().unwrap_or_default() == "Cargo.toml"
    };

    is_rs_file(&dir_entry) || is_package_json(&dir_entry)
}
