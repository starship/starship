use ansi_term::Color;
use std::fs::{self, File};
use std::io;
use tempfile;

use crate::common;
use crate::common::TestCommand;

/// Wrapper around common::render_module("nodejs_npm") to work around platform quirks
fn render_npm_module() -> std::process::Command {
    let mut command = common::render_module("nodejs_npm");

    // Appears to be the same issue as in the `nodejs` module.
    if cfg!(windows) {
        let system_root = std::env::var("SYSTEMROOT")
            .map(|i| {
                if i.trim().is_empty() {
                    "C:\\WINDOWS".into()
                } else {
                    i
                }
            })
            .unwrap_or_else(|_| "C:\\WINDOWS".into());
        command.env("SYSTEMROOT", system_root);
    }
    command
}

#[test]
fn folder_without_node_files() -> io::Result<()> {
    let dir = tempfile::tempdir()?;

    let output = render_npm_module()
        .use_config(toml::toml! {
            [nodejs_npm]
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

    let output = render_npm_module()
        .use_config(toml::toml! {
            [nodejs_npm]
            disabled = false
        })
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("with {} ", Color::Red.bold().paint("npm v6.9.0"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_js_file() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("index.js"))?.sync_all()?;

    let output = render_npm_module()
        .use_config(toml::toml! {
            [nodejs_npm]
            disabled = false
        })
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("with {} ", Color::Red.bold().paint("npm v6.9.0"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_node_modules() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let node_modules = dir.path().join("node_modules");
    fs::create_dir_all(&node_modules)?;

    let output = render_npm_module()
        .use_config(toml::toml! {
            [nodejs_npm]
            disabled = false
        })
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("with {} ", Color::Red.bold().paint("npm v6.9.0"));
    assert_eq!(expected, actual);
    Ok(())
}
