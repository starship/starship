use std::fs::File;
use std::io;

use ansi_term::Color;
use tempfile;

use crate::common;

#[test]
#[ignore]
fn folder_with_python_version() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join(".python-version"))?.sync_all()?;

    let output = common::render_module("python")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("🐍 v3.7.4"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_requirements_txt() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("requirements.txt"))?.sync_all()?;

    let output = common::render_module("python")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("🐍 v3.7.4"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_pyproject_toml() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("pyproject.toml"))?.sync_all()?;

    let output = common::render_module("python")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("🐍 v3.7.4"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_pipfile() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("Pipfile"))?.sync_all()?;

    let output = common::render_module("python")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("🐍 v3.7.4"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_tox() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("tox.ini"))?.sync_all()?;

    let output = common::render_module("python")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("🐍 v3.7.4"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_py_file() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("main.py"))?.sync_all()?;

    let output = common::render_module("python")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("🐍 v3.7.4"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn with_virtual_env() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("main.py"))?.sync_all()?;
    let output = common::render_module("python")
        .env("VIRTUAL_ENV", "/foo/bar/my_venv")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("🐍 v3.7.4 (my_venv)"));
    assert_eq!(expected, actual);
    Ok(())
}
