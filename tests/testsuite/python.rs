use std::fs::File;
use std::io;

use ansi_term::Color;

use crate::common;
use crate::common::TestCommand;

#[test]
#[ignore]
fn folder_with_python_version() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join(".python-version"))?;

    let output = common::render_module("python")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("🐍 v3.6.9"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_requirements_txt() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("requirements.txt"))?;

    let output = common::render_module("python")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("🐍 v3.6.9"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_pyproject_toml() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("pyproject.toml"))?;

    let output = common::render_module("python")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("🐍 v3.6.9"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_py_file() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("main.py"))?;

    let output = common::render_module("python")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("🐍 v3.6.9"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn with_virtual_env() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("main.py"))?;
    let output = common::render_module("python")
        .env("VIRTUAL_ENV", "/foo/bar/my_venv")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "via {} ",
        Color::Yellow.bold().paint("🐍 v3.6.9(my_venv)")
    );
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn with_pyenv() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("main.py"))?;
    let output = common::render_module("python")
        .use_config(toml::toml! {
            [python]
            pyenv_version_name = true
        })
        .env("VIRTUAL_ENV", "/foo/bar/my_venv")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!("via {} ", Color::Yellow.bold().paint("🐍 pyenv system"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn with_pyenv_no_output() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("main.py"))?;
    let output = common::render_module("python")
        .use_config(toml::toml! {
            [python]
            pyenv_version_name = true
        })
        .env("PATH", "")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!("", actual);
    Ok(())
}
