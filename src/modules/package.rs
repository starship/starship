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

// TODO: Combine into one function and just call for different file names!
fn has_rs_files(dir_entry: &PathBuf) -> bool {
    let is_cargo_toml =
        |d: &PathBuf| -> bool { d.is_file() && d.file_name().unwrap_or_default() == "Cargo.toml" };

    is_cargo_toml(&dir_entry)
}

fn has_js_files(dir_entry: &PathBuf) -> bool {
    let is_package_json = |d: &PathBuf| -> bool {
        d.is_file() && d.file_name().unwrap_or_default() == "package.json"
    };

    is_package_json(&dir_entry)
}

// TODO: Move to `utils.rs` file and import
fn read_file(file_name: String) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    data
}

fn extract_cargo_version() -> String {
    let data = read_file("Cargo.toml".to_string());

    let toml = data.parse::<toml::Value>().unwrap();
    let version = format_version(toml["package"]["version"].to_string());
    return version;
}

fn extract_package_version() -> String {
    let data = read_file("package.json".to_string());

    let json: serde_json::Value = serde_json::from_str(&data).unwrap();
    let version = format_version(json["version"].to_string());
    return version;
}

fn get_package_version(context: &Context) -> Option<String> {
    let is_rs_project = context.dir_files.iter().any(has_rs_files);
    if is_rs_project {
        return Some(extract_cargo_version());
    }

    let is_js_project = context.dir_files.iter().any(has_js_files);
    if is_js_project {
        return Some(extract_package_version());
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
