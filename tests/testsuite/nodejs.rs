use ansi_term::Color;
use std::fs::{self, File};
use std::io;

use crate::common;

fn render_node_module() -> std::process::Command {
    let mut command = common::render_module("nodejs");
    if cfg!(windows) {
        command
        .env("SYSTEMROOT", std::env::var("SYSTEMROOT").unwrap());
    }
    command
}

#[test]
fn folder_without_node_files() -> io::Result<()> {
    let dir = common::new_tempdir()?;

    let output = render_node_module()
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
    File::create(dir.path().join("package.json"))?.sync_all()?;

    let output = render_node_module()
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
    File::create(dir.path().join("index.js"))?.sync_all()?;

    let output = render_node_module()
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

    let output = render_node_module()
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Green.bold().paint("⬢ v12.0.0"));
    assert_eq!(expected, actual);
    Ok(())
}
