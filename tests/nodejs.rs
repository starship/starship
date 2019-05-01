use ansi_term::Color;
use starship::segment::Segment;
use std::fs::{self, File};
use std::io;
use tempfile::TempDir;

mod common;

#[test]
#[ignore]
fn folder_with_package_json() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join("package.json"))?;

    let expected = format!(
        "via {} ",
        Segment::new("node")
            .set_value("⬢ v12.0.0")
            .set_style(Color::Green.bold())
    );
    let actual = common::render_module("nodejs", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_js_file() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join("index.js"))?;

    let expected = format!(
        "via {} ",
        Segment::new("node")
            .set_value("⬢ v12.0.0")
            .set_style(Color::Green.bold())
    );
    let actual = common::render_module("nodejs", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_node_modules() -> io::Result<()> {
    let dir = TempDir::new()?;
    let node_modules = dir.path().join("node_modules");
    fs::create_dir_all(&node_modules)?;

    let expected = format!(
        "via {} ",
        Segment::new("node")
            .set_value("⬢ v12.0.0")
            .set_style(Color::Green.bold())
    );
    let actual = common::render_module("nodejs", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}
