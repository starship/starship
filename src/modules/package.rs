use super::Segment;
use crate::context::Context;
use ansi_term::Color;
use serde_json;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml;

/// Creates a segment with the current package version
pub fn segment(context: &Context) -> Option<Segment> {
    match get_package_version(context) {
        Some(package_version) => {
            const RUST_CHAR: &str = "📦";
            const SEGMENT_COLOR: Color = Color::Red;

            let mut segment = Segment::new("package");
            // TODO: Should have prefix "is "
            segment.set_style(SEGMENT_COLOR);

            segment.set_value(format!("{} {}", RUST_CHAR, package_version));

            Some(segment)
        }
        None => None,
    }
}

fn has_rs_files(dir_entry: &PathBuf) -> bool {
    let is_rs_file =
        |d: &PathBuf| -> bool { d.is_file() && d.extension().unwrap_or_default() == "rs" };
    let is_cargo_toml =
        |d: &PathBuf| -> bool { d.is_file() && d.file_name().unwrap_or_default() == "Cargo.toml" };

    is_rs_file(&dir_entry) || is_cargo_toml(&dir_entry)
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

fn get_package_version(context: &Context) -> Option<String> {
    let is_rs_project = context.dir_files.iter().any(has_rs_files);
    if !is_rs_project {
        let is_js_project = context.dir_files.iter().any(has_js_files);
        if !is_js_project {
            return None;
        } else {
            let mut file = File::open("package.json").unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();

            let json: serde_json::Value = serde_json::from_str(&data).unwrap();
            let version = format_version(json["version"].to_string());
            return Some(version);
        }
    } else {
        let mut file = File::open("Cargo.toml").unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        let toml = data.parse::<toml::Value>().unwrap();
        let version = format_version(toml["package"]["version"].to_string());
        return Some(version);
    }
}

fn format_version(version: String) -> String {
    format!("v{}", version.replace('"', "").trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_version() {
        let input = String::from("0.1.0");
        assert_eq!(format_version(input), "v0.1.0");
    }
}
