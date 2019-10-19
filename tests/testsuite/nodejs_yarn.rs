use ansi_term::Color;
use std::fs::{self, File};
use std::io;
use tempfile;

use crate::common;
use crate::common::TestCommand;

#[test]
fn folder_without_node_files() -> io::Result<()> {
    let dir = tempfile::tempdir()?;

    let output = common::render_module("nodejs_yarn")
        .use_config(toml::toml! {
            [nodejs_yarn]
            disabled = false
        })
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_package_json() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("package.json"))?.sync_all()?;

    let output = common::render_module("nodejs_yarn")
        .use_config(toml::toml! {
            [nodejs_yarn]
            disabled = false
        })
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("with {} ", Color::Blue.bold().paint("🐈 v1.19.1"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_js_file() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("index.js"))?.sync_all()?;

    let output = common::render_module("nodejs_yarn")
        .use_config(toml::toml! {
            [nodejs_yarn]
            disabled = false
        })
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("with {} ", Color::Blue.bold().paint("🐈 v1.19.1"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_node_modules() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let node_modules = dir.path().join("node_modules");
    fs::create_dir_all(&node_modules)?;

    let output = common::render_module("nodejs_yarn")
        .use_config(toml::toml! {
            [nodejs_yarn]
            disabled = false
        })
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("with {} ", Color::Blue.bold().paint("🐈 v1.19.1"));
    assert_eq!(expected, actual);
    Ok(())
}
