use ansi_term::Color;
use std::fs::{self, File};
use std::io::{self, Write};
use tempfile;

use crate::common;

#[test]
fn folder_without_dotterraform() -> io::Result<()> {
    let dir = tempfile::tempdir()?;

    let output = common::render_module("terraform")
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
fn folder_with_dotterraform_no_environment() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let tf_dir = dir.path().join(".terraform");
    fs::create_dir(&tf_dir)?;

    let output = common::render_module("terraform")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Fixed(105).bold().paint("💠 default"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_dotterraform_with_environment() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let tf_dir = dir.path().join(".terraform");
    fs::create_dir(&tf_dir)?;
    let mut file = File::create(tf_dir.join("environment"))?;
    file.write_all(b"development")?;
    file.sync_all()?;

    let output = common::render_module("terraform")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Fixed(105).bold().paint("💠 development"));
    assert_eq!(expected, actual);
    Ok(())
}
