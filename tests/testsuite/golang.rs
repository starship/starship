use ansi_term::Color;
use std::fs::{self, File};
use std::io;

use crate::common;

#[test]
fn folder_without_go_files() -> io::Result<()> {
    let dir = common::new_tempdir()?;

    let output = common::render_module("golang")
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
fn folder_with_go_file() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("main.go"))?.sync_all()?;

    let output = common::render_module("golang")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_go_mod() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("go.mod"))?.sync_all()?;

    let output = common::render_module("golang")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_go_sum() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("go.sum"))?.sync_all()?;

    let output = common::render_module("golang")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_godeps() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    let godeps = dir.path().join("Godeps");
    fs::create_dir_all(&godeps)?;

    let output = common::render_module("golang")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_glide_yaml() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("glide.yaml"))?.sync_all()?;

    let output = common::render_module("golang")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_gopkg_yml() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("Gopkg.yml"))?.sync_all()?;

    let output = common::render_module("golang")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_gopkg_lock() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("Gopkg.lock"))?.sync_all()?;

    let output = common::render_module("golang")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1"));
    assert_eq!(expected, actual);
    Ok(())
}
