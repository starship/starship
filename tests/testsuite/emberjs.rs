use ansi_term::Color;
use std::fs;
use std::io;
use tempfile;

use crate::common;

#[test]
fn without_node_modules() -> io::Result<()> {
    let dir = tempfile::tempdir()?;

    let output = common::render_module("emberjs")
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

    let output = common::render_module("emberjs")
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

    let output = common::render_module("emberjs")
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

    let output = common::render_module("emberjs")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("with {} ", Color::Red.bold().paint("🐹 v3.14.0"));
    assert_eq!(expected, actual);
    Ok(())
}
