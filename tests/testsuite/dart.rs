use ansi_term::Color;
use std::fs::File;
use std::io;
use tempfile;

use crate::common;

#[test]
fn folder_without_dart_files() -> io::Result<()> {
    let dir = tempfile::tempdir()?;

    let output = common::render_module("dart")
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
fn folder_with_pubspec_yaml() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("pubspec.yaml"))?.sync_all()?;

    let output = common::render_module("dart")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Blue.bold().paint("🎯 v2.5.2"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_dart_file() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("main.dart"))?.sync_all()?;

    let output = common::render_module("dart")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Blue.bold().paint("🎯 v2.5.2"));
    assert_eq!(expected, actual);
    Ok(())
}
