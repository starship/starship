use super::{Context, Module};
use crate::utils;

use ansi_term::Color;
use serde_json as json;
use toml;

/// Creates a segment with the current package version
///
/// Will display if a version is defined for your Node.js or Rust project (if one exists)
pub fn segment(_context: &Context) -> Option<Module> {
    match get_package_version() {
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

fn extract_cargo_version(file_contents: &str) -> Option<String> {
    let cargo_toml: toml::Value = toml::from_str(&file_contents).ok()?;
    let raw_version = cargo_toml.get("package")?.get("version")?.as_str()?;

    let formatted_version = format_version(raw_version);
    Some(formatted_version)
}

fn extract_package_version(file_contents: &str) -> Option<String> {
    let package_json: json::Value = json::from_str(&file_contents).ok()?;
    let raw_version = package_json.get("version")?.as_str()?;
    if raw_version == "null" {
        return None;
    };

    let formatted_version = format_version(raw_version);
    Some(formatted_version)
}

fn get_package_version() -> Option<String> {
    let cargo_toml = utils::read_file("Cargo.toml");
    if let Ok(cargo_toml) = cargo_toml {
        return extract_cargo_version(&cargo_toml);
    }

    let package_json = utils::read_file("package.json");
    if let Ok(package_json) = package_json {
        return extract_package_version(&package_json);
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
        let cargo_with_version = r#"
            [package]
            name = "starship"
            version = "0.1.0"
            "#;

        let expected_version = Some("v0.1.0".to_string());
        assert_eq!(extract_cargo_version(&cargo_with_version), expected_version);

        let cargo_without_version = r#"
            [package]
            name = "starship"
            "#;


        let expected_version = None;
        assert_eq!(
            extract_cargo_version(&cargo_without_version),
            expected_version
        );
    }

    #[test]
    fn test_extract_package_version() {
        let package_with_version = json::json!({
            "name": "spacefish",
            "version": "0.1.0"
        })
        .to_string();

        let expected_version = Some("v0.1.0".to_string());
        assert_eq!(
            extract_package_version(&package_with_version),
            expected_version
        );

        let package_without_version = json::json!({
            "name": "spacefish"
        })
        .to_string();

        let expected_version = None;
        assert_eq!(
            extract_package_version(&package_without_version),
            expected_version
        );
    }
}
