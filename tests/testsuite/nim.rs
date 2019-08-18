use ansi_term::Color;
use std::fs::{self, File};
use std::io;

use crate::common;

#[test]
fn folder_without_nim_files() -> io::Result<()> {
    let dir = common::new_tempdir()?;

    let output = common::render_module("nim")
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
fn folder_with_nimble_file() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("any.nimble"))?;

    let output = common::render_module("nim")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("👑 v0.20.0"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_nim_file() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("any.nim"))?;

    let output = common::render_module("nim")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("👑 v0.20.0"));
    assert_eq!(expected, actual);
    Ok(())
}
