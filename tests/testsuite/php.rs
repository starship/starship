use std::fs::File;
use std::io;

use ansi_term::Color;

use crate::common;
use crate::common::TestCommand;

#[test]
fn folder_without_php_files() -> io::Result<()> {
    let dir = common::new_tempdir()?;

    let output = common::render_module("php")
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
fn folder_with_composer_file() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("composer.json"))?;

    let output = common::render_module("php")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Fixed(147).bold().paint("🐘 v7.3.8"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_php_file() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("any.php"))?;

    let output = common::render_module("php")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Fixed(147).bold().paint("🐘 v7.3.8"));
    assert_eq!(expected, actual);
    Ok(())
}
