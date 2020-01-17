use ansi_term::Color;
use std::fs::File;
use std::io;
use tempfile;

use crate::common;

#[test]
fn folder_without_ruby_files() -> io::Result<()> {
    let dir = tempfile::tempdir()?;

    let output = common::render_module("ruby")
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
fn folder_with_gemfile() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("Gemfile"))?.sync_all()?;

    let output = common::render_module("ruby")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("💎 v2.6.3"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_rb_file() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("any.rb"))?.sync_all()?;

    let output = common::render_module("ruby")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("💎 v2.6.3"));
    assert_eq!(expected, actual);
    Ok(())
}
