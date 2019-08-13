use ansi_term::Color;
use std::fs::{self, File};
use std::io;

use crate::common::{self, TestCommand};

#[test]
fn folder_without_node_files() -> io::Result<()> {
    let dir = common::new_tempdir()?;

    let output = common::render_module("nodejs")
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
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("package.json"))?;

    let output = common::render_module("nodejs")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_js_file() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("index.js"))?;

    let output = common::render_module("nodejs")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_node_modules() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    let node_modules = dir.path().join("node_modules");
    fs::create_dir_all(&node_modules)?;

    let output = common::render_module("nodejs")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn config_disabled() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("package.json"))?;

    let output = common::render_module("nodejs")
        .use_config(toml::toml! {
            [nodejs]
            disabled = true
        })
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}
