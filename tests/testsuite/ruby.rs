use ansi_term::Color;
use std::fs::File;
use std::io;

use crate::common;

#[test]
fn folder_without_ruby_files() -> io::Result<()> {
    let dir = common::new_tempdir()?;

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
fn folder_with_gemfile() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("Gemfile"))?;

    let output = common::render_module("ruby")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("💎 v2.5.5"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn folder_with_rb_file() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("any.rb"))?;

    let output = common::render_module("ruby")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("💎 v2.5.5"));
    assert_eq!(expected, actual);
    Ok(())
}
