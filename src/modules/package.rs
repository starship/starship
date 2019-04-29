use super::Segment;
use crate::context::Context;
use ansi_term::Color;
use serde_json;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml;

/// Creates a segment with the current package version
///
/// Will display if a version is defined for your Node.js or Rust project (if one exists)
pub fn segment(context: &Context) -> Option<Segment> {
    match get_package_version(context) {
        Some(package_version) => {
            const PACKAGE_CHAR: &str = "📦";
            const SEGMENT_COLOR: Color = Color::Red;

            // TODO: Make the prefix for the module "is "
            let mut segment = Segment::new("package");
            segment.set_style(SEGMENT_COLOR.bold());

            segment.set_value(format!("{} {}", PACKAGE_CHAR, package_version));

            Some(segment)
        }
        None => None,
    }
}

// TODO: Combine into one function and just call for different file names!
fn is_cargo_toml(dir_entry: &PathBuf) -> bool {
    dir_entry.is_file() && dir_entry.file_name().unwrap_or_default() == "Cargo.toml"
}

fn is_package_json(dir_entry: &PathBuf) -> bool {
    dir_entry.is_file() && dir_entry.file_name().unwrap_or_default() == "package.json"
}

// TODO: Move to `utils.rs` file and import
fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    data
}

fn extract_cargo_version() -> Option<String> {
    let data = read_file("Cargo.toml");

    let toml = data.parse::<toml::Value>().unwrap();
    match toml["package"]["version"].as_str() {
        Some(raw_version) => {
            let version = format_version(raw_version.to_string());
            Some(version)
        }
        None => None,
    }
}

fn extract_package_version() -> Option<String> {
    let data = read_file("package.json");

    let json: serde_json::Value = serde_json::from_str(&data).unwrap();
    let raw_version = json["version"].to_string();
    if raw_version == "null" {
        return None;
    }
    let version = format_version(raw_version);
    return Some(version);
}

fn get_package_version(context: &Context) -> Option<String> {
    let has_cargo_toml = context.dir_files.iter().any(is_cargo_toml);
    if has_cargo_toml {
        return extract_cargo_version();
    }

    let has_package_json = context.dir_files.iter().any(is_package_json);
    if has_package_json {
        return extract_package_version();
    }

    return None;
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
