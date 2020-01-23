use ansi_term::Color;
use std::fs::{self, File};
use std::io::{self, Write};
use tempfile;

use crate::common;
use crate::common::TestCommand;

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
fn folder_with_tf_file() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("main.tf"))?;

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
fn folder_with_workspace_override() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("main.tf"))?;

    let output = common::render_module("terraform")
        .arg("--path")
        .arg(dir.path())
        .env_clear()
        .env("TF_WORKSPACE", "development")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Fixed(105).bold().paint("💠 development"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_datadir_override() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("main.tf"))?;

    let datadir = tempfile::tempdir()?;
    let mut file = File::create(datadir.path().join("environment"))?;
    file.write_all(b"development")?;
    file.sync_all()?;

    let output = common::render_module("terraform")
        .arg("--path")
        .arg(dir.path())
        .env_clear()
        .env("TF_DATA_DIR", datadir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Fixed(105).bold().paint("💠 development"));
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

#[test]
#[ignore]
fn folder_with_dotterraform_with_version_no_environment() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let tf_dir = dir.path().join(".terraform");
    fs::create_dir(&tf_dir)?;

    let output = common::render_module("terraform")
        .arg("--path")
        .arg(dir.path())
        .use_config(toml::toml! {
            [terraform]
            show_version = true
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "via {} ",
        Color::Fixed(105).bold().paint("💠 v0.12.14 default")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_dotterraform_with_version_with_environment() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let tf_dir = dir.path().join(".terraform");
    fs::create_dir(&tf_dir)?;
    let mut file = File::create(tf_dir.join("environment"))?;
    file.write_all(b"development")?;
    file.sync_all()?;

    let output = common::render_module("terraform")
        .arg("--path")
        .arg(dir.path())
        .use_config(toml::toml! {
            [terraform]
            show_version = true
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "via {} ",
        Color::Fixed(105).bold().paint("💠 v0.12.14 development")
    );
    assert_eq!(expected, actual);
    Ok(())
}
