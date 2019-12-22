use ansi_term::Color;
use std::collections::HashMap;
use std::fs;
use std::io;

use crate::common;

use tempfile;

use starship::config::{ModuleConfig, RootModuleConfig, SegmentConfig};
use starship::configs::js_library::{JsLibraryConfig, JsLibraryConfigItem};

#[test]
fn parse_and_merge_config() {
    let config = toml::toml! {
        suffix = "!!"

        [map.emberjs]
        disabled = true

        [map.react]
        dependency = "react-dom"
        style = "bold purple"
        symbol = "🥳 "

        [map.leaflet]
        dependency = "leaflet"
        style = "bold green"
        symbol = "🍃 "
    };

    let actual = JsLibraryConfig::new().load_config(&config);

    let expected = JsLibraryConfig {
        map: {
            let mut map = HashMap::new();
            map.insert(
                "emberjs".to_string(),
                JsLibraryConfigItem {
                    dependency: "ember-source",
                    symbol: SegmentConfig::new("🐹 "),
                    style: Some(Color::Red.bold()),
                    disabled: true,
                },
            );
            map.insert(
                "react".to_string(),
                JsLibraryConfigItem {
                    dependency: "react-dom",
                    symbol: SegmentConfig::new("🥳 "),
                    style: Some(Color::Purple.bold()),
                    disabled: false,
                },
            );
            map.insert(
                "leaflet".to_string(),
                JsLibraryConfigItem {
                    dependency: "leaflet",
                    symbol: SegmentConfig::new("🍃 "),
                    style: Some(Color::Green.bold()),
                    disabled: false,
                },
            );
            map
        },
        prefix: "with ",
        suffix: "!!",
        separator: " and ",
    };

    assert_eq!(actual, expected);
}

#[test]
fn without_node_modules() -> io::Result<()> {
    let dir = tempfile::tempdir()?;

    let output = common::render_module("js_library")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn without_ember_source_package() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    fs::create_dir(dir.path().join("node_modules"))?;

    let output = common::render_module("js_library")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn without_ember_source_package_json() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    fs::create_dir_all(dir.path().join("node_modules").join("ember-source"))?;

    let output = common::render_module("js_library")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn with_ember_source_package() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let pkg_path = dir.path().join("node_modules").join("ember-source");
    fs::create_dir_all(&pkg_path)?;
    let pkg_json_path = pkg_path.join("package.json");
    fs::write(&pkg_json_path, "{\n  \"version\": \"3.14.0\"\n}")?;

    let output = common::render_module("js_library")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("with 🐹 {} ", Color::Red.bold().paint("v3.14.0"));
    assert_eq!(expected, actual);
    Ok(())
}
