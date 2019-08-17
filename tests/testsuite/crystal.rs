use ansi_term::Color;
use std::fs::{self, File};
use std::io;

use crate::common;

#[test]
fn folder_without_crystal_files() -> io::Result<()> {
    let dir = common::new_tempdir()?;

    let output = common::render_module("crystal")
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
fn folder_with_shardfile() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("shard.yml"))?;

    let output = common::render_module("crystal")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("💎 v0.30.1"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_cr_file() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("any.cr"))?;

    let output = common::render_module("crystal")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("💎 v0.30.1"));
    assert_eq!(expected, actual);
    Ok(())
}
