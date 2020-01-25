use ansi_term::Color;
use std::fs::{self, File};
use std::io;
use tempfile;

use crate::common;

#[test]
fn folder_without_elm() -> io::Result<()> {
    let dir = tempfile::tempdir()?;

    let output = common::render_module("elm")
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
#[cfg(not(windows))]
fn folder_with_elm_json() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("elm.json"))?.sync_all()?;

    let output = common::render_module("elm")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Cyan.bold().paint("🌳 v0.19.1"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
#[cfg(not(windows))]
fn folder_with_elm_package_json() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("elm-package.json"))?.sync_all()?;

    let output = common::render_module("elm")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Cyan.bold().paint("🌳 v0.19.1"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
#[cfg(not(windows))]
fn folder_with_elm_stuff_directory() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let elmstuff = dir.path().join("elm-stuff");
    fs::create_dir_all(&elmstuff)?;

    let output = common::render_module("elm")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Cyan.bold().paint("🌳 v0.19.1"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
#[cfg(not(windows))]
fn folder_with_elm_file() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("main.elm"))?.sync_all()?;

    let output = common::render_module("elm")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Cyan.bold().paint("🌳 v0.19.1"));
    assert_eq!(expected, actual);
    Ok(())
}
