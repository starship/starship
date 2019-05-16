use ansi_term::Color;
use starship::segment::Segment;
use std::fs::File;
use std::io;
use tempfile::TempDir;

mod common;

#[test]
#[ignore]
fn folder_with_python_version() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join(".python-version"))?;

    let expected = format!(
        "via {} ",
        Segment::new("python")
            .set_value("🐍 v3.6.8")
            .set_style(Color::Yellow.bold())
    );
    let actual = common::render_module("python", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_requirements_txt() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join("requirements.txt"))?;

    let expected = format!(
        "via {} ",
        Segment::new("python")
            .set_value("🐍 v3.6.8")
            .set_style(Color::Yellow.bold())
    );
    let actual = common::render_module("python", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_pyproject_toml() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join("pyproject.toml"))?;

    let expected = format!(
        "via {} ",
        Segment::new("python")
            .set_value("🐍 v3.6.8")
            .set_style(Color::Yellow.bold())
    );
    let actual = common::render_module("python", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn folder_with_py_file() -> io::Result<()> {
    let dir = TempDir::new()?;
    File::create(dir.path().join("main.py"))?;

    let expected = format!(
        "via {} ",
        Segment::new("python")
            .set_value("🐍 v3.6.8")
            .set_style(Color::Yellow.bold())
    );
    let actual = common::render_module("python", &dir.path());
    assert_eq!(expected, actual);

    Ok(())
}
