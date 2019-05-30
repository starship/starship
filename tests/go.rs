use ansi_term::Color;
use std::fs::{self, File};
use std::io;

mod common;

#[test]
#[ignore]
fn folder_with_go_file() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("main.go"))?;

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.10"));
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_go_mod() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("go.mod"))?;

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.10"));
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_go_sum() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("go.sum"))?;

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.10"));
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_godeps() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    let godeps = dir.path().join("Godeps");
    fs::create_dir_all(&godeps)?;

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.10"));
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_glide_yaml() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("glide.yaml"))?;

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.10"));
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_gopkg_yml() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("Gopkg.yml"))?;

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.10"));
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_gopkg_lock() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("Gopkg.lock"))?;

    let expected = format!("via {} ", Color::Cyan.bold().paint("🐹 v1.10"));
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}
