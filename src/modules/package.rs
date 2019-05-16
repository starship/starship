use super::{Context, Module};

use ansi_term::Color;
use serde_json;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use toml;

/// Creates a segment with the current package version
///
/// Will display if a version is defined for your Node.js or Rust project (if one exists)
pub fn segment(context: &Context) -> Option<Module> {
    match get_package_version(context) {
        Some(package_version) => {
            const PACKAGE_CHAR: &str = "📦 ";
            let module_color = Color::Red.bold();

            let mut module = Module::new("package");
            module.set_style(module_color);
            module.get_prefix().set_value("is ");

            module.new_segment("symbol", PACKAGE_CHAR);
            module.new_segment("version", package_version);

            Some(module)
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
fn read_file(file_name: &str) -> io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;

    Ok(data)
}

fn extract_cargo_version(file_contents: &str) -> Option<String> {
    let cargo_toml = file_contents.parse::<toml::Value>().ok()?;
    let raw_version = cargo_toml.get("package")?.get("version")?.as_str()?;

    let formatted_version = format_version(raw_version);
    Some(formatted_version)
}

fn extract_package_version(file_contents: &str) -> Option<String> {
    let package_json: serde_json::Value = serde_json::from_str(&file_contents).ok()?;
    let raw_version = package_json.get("version")?.as_str()?;
    if raw_version == "null" {
        return None;
    };

    let formatted_version = format_version(raw_version);
    Some(formatted_version)
}

fn get_package_version(context: &Context) -> Option<String> {
    let has_cargo_toml = context.dir_files.iter().any(is_cargo_toml);
    if has_cargo_toml {
        let file_contents = read_file("Cargo.toml").ok()?;
        return extract_cargo_version(&file_contents);
    }

    let has_package_json = context.dir_files.iter().any(is_package_json);
    if has_package_json {
        let file_contents = read_file("package.json").ok()?;
        return extract_package_version(&file_contents);
    }

    None
}

fn format_version(version: &str) -> String {
    format!("v{}", version.replace('"', "").trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_version() {
        assert_eq!(format_version("0.1.0"), "v0.1.0");
    }

    #[test]
    fn test_extract_cargo_version() {
        let cargo_with_version = "[package]
            name = \"starship\"
            version = \"0.1.0\""; 
            
        let expected_version = Some("v0.1.0".to_string());
        assert_eq!(extract_cargo_version(&cargo_with_version), expected_version);

        let cargo_without_version = "[package]
            name = \"starship\""; 
        
        let expected_version = None;
        assert_eq!(extract_cargo_version(&cargo_without_version), expected_version);
    }

    #[test]
    fn test_extract_package_version() {
        let package_with_version = serde_json::json!({
            "name": "spacefish",
            "version": "0.1.0"
        }).to_string();

        let expected_version = Some("v0.1.0".to_string());
        assert_eq!(extract_package_version(&package_with_version), expected_version);

        let package_without_version = serde_json::json!({
            "name": "spacefish"
        }).to_string();
        
        let expected_version = None;
        assert_eq!(extract_package_version(&package_without_version), expected_version);
    }
}
