use ansi_term::Color;
use starship::segment::Segment;
use std::fs::{self, File};
use std::io;
use tempfile::TempDir;

mod common;

#[test]
#[ignore]
fn folder_with_go_file() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join("main.go"))?;

    let expected = format!(
        "via {} ",
        Segment::new("go")
            .set_value("🐹 v1.12.4")
            .set_style(Color::Cyan.bold())
    );
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_go_mod() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join("go.mod"))?;

    let expected = format!(
        "via {} ",
        Segment::new("go")
            .set_value("🐹 v1.12.4")
            .set_style(Color::Cyan.bold())
    );
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_go_sum() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join("go.sum"))?;

    let expected = format!(
        "via {} ",
        Segment::new("go")
            .set_value("🐹 v1.12.4")
            .set_style(Color::Cyan.bold())
    );
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_godeps() -> io::Result<()> {
    let dir = TempDir::new()?;
    let godeps = dir.path().join("Godeps");
    fs::create_dir_all(&godeps)?;

    let expected = format!(
        "via {} ",
        Segment::new("go")
            .set_value("🐹 v1.12.4")
            .set_style(Color::Cyan.bold())
    );
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_glide_yaml() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join("glide.yaml"))?;

    let expected = format!(
        "via {} ",
        Segment::new("go")
            .set_value("🐹 v1.12.4")
            .set_style(Color::Cyan.bold())
    );
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_gopkg_yml() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join("Gopkg.yml"))?;

    let expected = format!(
        "via {} ",
        Segment::new("go")
            .set_value("🐹 v1.12.4")
            .set_style(Color::Cyan.bold())
    );
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_gopkg_lock() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join("Gopkg.lock"))?;

    let expected = format!(
        "via {} ",
        Segment::new("go")
            .set_value("🐹 v1.12.4")
            .set_style(Color::Cyan.bold())
    );
    let actual = common::render_module("go", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}
