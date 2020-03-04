use std::path::PathBuf;

use super::{Context, Module};
use crate::utils;

use regex::Regex;
use serde_json as json;
use toml;

use super::{RootModuleConfig, SegmentConfig};
use crate::configs::package::PackageConfig;

/// Creates a module with the current package version
///
/// Will display if a version is defined for your Node.js or Rust project (if one exists)
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    match get_package_version(&context.current_dir) {
        Some(package_version) => {
            let mut module = context.new_module("package");
            let config: PackageConfig = PackageConfig::try_load(module.config);

            module.set_style(config.style);
            module.get_prefix().set_value("is ");

            module.create_segment("symbol", &config.symbol);
            module.create_segment("version", &SegmentConfig::new(&package_version));

            Some(module)
        }
        None => None,
    }
}

fn extract_cargo_version(file_contents: &str) -> Option<String> {
    let cargo_toml: toml::Value = toml::from_str(file_contents).ok()?;
    let raw_version = cargo_toml.get("package")?.get("version")?.as_str()?;

    let formatted_version = format_version(raw_version);
    Some(formatted_version)
}

fn extract_package_version(file_contents: &str) -> Option<String> {
    let package_json: json::Value = json::from_str(file_contents).ok()?;

    if package_json.get("private").and_then(json::Value::as_bool) == Some(true) {
        return None;
    }

    let raw_version = package_json.get("version")?.as_str()?;
    if raw_version == "null" {
        return None;
    };

    let formatted_version = format_version(raw_version);
    Some(formatted_version)
}

fn extract_poetry_version(file_contents: &str) -> Option<String> {
    let poetry_toml: toml::Value = toml::from_str(file_contents).ok()?;
    let raw_version = poetry_toml
        .get("tool")?
        .get("poetry")?
        .get("version")?
        .as_str()?;

    let formatted_version = format_version(raw_version);
    Some(formatted_version)
}

fn extract_gradle_version(file_contents: &str) -> Option<String> {
    let re = Regex::new(r#"(?m)^version ['"](?P<version>[^'"]+)['"]$"#).unwrap();
    let caps = re.captures(file_contents)?;

    let formatted_version = format_version(&caps["version"]);
    Some(formatted_version)
}

fn extract_composer_version(file_contents: &str) -> Option<String> {
    let composer_json: json::Value = json::from_str(file_contents).ok()?;
    let raw_version = composer_json.get("version")?.as_str()?;
    if raw_version == "null" {
        return None;
    };

    let formatted_version = format_version(raw_version);
    Some(formatted_version)
}

fn get_package_version(base_dir: &PathBuf) -> Option<String> {
    if let Ok(cargo_toml) = utils::read_file(base_dir.join("Cargo.toml")) {
        extract_cargo_version(&cargo_toml)
    } else if let Ok(package_json) = utils::read_file(base_dir.join("package.json")) {
        extract_package_version(&package_json)
    } else if let Ok(poetry_toml) = utils::read_file(base_dir.join("pyproject.toml")) {
        extract_poetry_version(&poetry_toml)
    } else if let Ok(composer_json) = utils::read_file(base_dir.join("composer.json")) {
        extract_composer_version(&composer_json)
    } else if let Ok(build_gradle) = utils::read_file(base_dir.join("build.gradle")) {
        extract_gradle_version(&build_gradle)
    } else {
        None
    }
}

fn format_version(version: &str) -> String {
    let cleaned = version.replace('"', "").trim().to_string();
    if cleaned.starts_with('v') {
        cleaned
    } else {
        format!("v{}", cleaned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_version() {
        assert_eq!(format_version("0.1.0"), "v0.1.0");
        assert_eq!(format_version(" 0.1.0 "), "v0.1.0");
        assert_eq!(format_version("0.1.0 "), "v0.1.0");
        assert_eq!(format_version(" 0.1.0"), "v0.1.0");
        assert_eq!(format_version("\"0.1.0\""), "v0.1.0");

        assert_eq!(format_version("v0.1.0"), "v0.1.0");
        assert_eq!(format_version(" v0.1.0 "), "v0.1.0");
        assert_eq!(format_version(" v0.1.0"), "v0.1.0");
        assert_eq!(format_version("v0.1.0 "), "v0.1.0");
        assert_eq!(format_version("\"v0.1.0\""), "v0.1.0");
    }

    #[test]
    fn test_extract_cargo_version() {
        let cargo_with_version = toml::toml! {
            [package]
            name = "starship"
            version = "0.1.0"
        }
        .to_string();

        let expected_version = Some("v0.1.0".to_string());
        assert_eq!(extract_cargo_version(&cargo_with_version), expected_version);

        let cargo_without_version = toml::toml! {
            [package]
            name = "starship"
        }
        .to_string();

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

        let package_with_null_version = json::json!({
            "name": "spacefish",
            "version": null
        })
        .to_string();

        let expected_version = None;
        assert_eq!(
            extract_package_version(&package_with_null_version),
            expected_version
        );

        let package_with_null_string_version = json::json!({
            "name": "spacefish",
            "version": "null"
        })
        .to_string();

        let expected_version = None;
        assert_eq!(
            extract_package_version(&package_with_null_string_version),
            expected_version
        );

        let private_package = json::json!({
            "name": "spacefish",
            "version": "0.1.0",
            "private": true
        })
        .to_string();

        let expected_version = None;
        assert_eq!(extract_package_version(&private_package), expected_version);
    }

    #[test]
    fn test_extract_poetry_version() {
        let poetry_with_version = toml::toml! {
            [tool.poetry]
            name = "starship"
            version = "0.1.0"
        }
        .to_string();

        let expected_version = Some("v0.1.0".to_string());
        assert_eq!(
            extract_poetry_version(&poetry_with_version),
            expected_version
        );

        let poetry_without_version = toml::toml! {
            [tool.poetry]
            name = "starship"
        }
        .to_string();

        let expected_version = None;
        assert_eq!(
            extract_poetry_version(&poetry_without_version),
            expected_version
        );
    }

    #[test]
    fn test_extract_gradle_version() {
        let gradle_single_quotes = "plugins {
    id 'java'
    id 'test.plugin' version '0.2.0'
}
version '0.1.0'
java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}";

        let expected_version = Some("v0.1.0".to_string());
        assert_eq!(
            extract_gradle_version(&gradle_single_quotes),
            expected_version
        );

        let gradle_double_quotes = "plugins {
    id 'java'
    id 'test.plugin' version '0.2.0'
}
version \"0.1.0\"
java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}";

        let expected_version = Some("v0.1.0".to_string());
        assert_eq!(
            extract_gradle_version(&gradle_double_quotes),
            expected_version
        );

        let gradle_release_candidate = "plugins {
    id 'java'
    id 'test.plugin' version '0.2.0'
}
version '0.1.0-rc1'
java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}";

        let expected_version = Some("v0.1.0-rc1".to_string());
        assert_eq!(
            extract_gradle_version(&gradle_release_candidate),
            expected_version
        );

        let gradle_without_version = "plugins {
    id 'java'
    id 'test.plugin' version '0.2.0'
}
java {
    sourceCompatibility = JavaVersion.VERSION_1_8
    targetCompatibility = JavaVersion.VERSION_1_8
}";

        let expected_version = None;
        assert_eq!(
            extract_gradle_version(&gradle_without_version),
            expected_version
        );
    }

    #[test]
    fn test_extract_composer_version() {
        let composer_with_version = json::json!({
            "name": "spacefish",
            "version": "0.1.0"
        })
        .to_string();

        let expected_version = Some("v0.1.0".to_string());
        assert_eq!(
            extract_composer_version(&composer_with_version),
            expected_version
        );

        let composer_without_version = json::json!({
            "name": "spacefish"
        })
        .to_string();

        let expected_version = None;
        assert_eq!(
            extract_composer_version(&composer_without_version),
            expected_version
        );
    }
}
