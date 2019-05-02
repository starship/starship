use ansi_term::Color;
use starship::segment::Segment;
use std::fs::File;
use std::io;
use tempfile::TempDir;

mod common;

#[test]
#[ignore]
fn folder_with_cargo_toml() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join("Cargo.toml"))?;

    let expected = format!(
        "via {} ",
        Segment::new("rust")
            .set_value("🦀 v1.34.1")
            .set_style(Color::Red.bold())
    );
    let actual = common::render_module("rust", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_rs_file() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join("main.rs"))?;

    let expected = format!(
        "via {} ",
        Segment::new("rust")
            .set_value("🦀 v1.34.1")
            .set_style(Color::Red.bold())
    );
    let actual = common::render_module("rust", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}
