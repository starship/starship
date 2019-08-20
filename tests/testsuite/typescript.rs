use ansi_term::Color;
use std::fs::{self, File};
use std::io;

use crate::common;

#[test]
#[ignore]
fn folder_without_typescript_files() -> io::Result<()> {
    let dir = common::new_tempdir()?;

    let output = common::render_module("typescript")
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
fn folder_with_tsconfig() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("tsconfig.json"))?;

    let output = common::render_module("typescript")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Blue.bold().paint("■ v3.5.3"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_ts_file() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("any.ts"))?;

    let output = common::render_module("typescript")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Blue.bold().paint("■ v3.5.3"));
    assert_eq!(expected, actual);
    Ok(())
}
