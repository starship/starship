use ansi_term::Color;
use std::fs::File;
use std::io;

use crate::common;

#[test]
fn folder_without_docker_files() -> io::Result<()> {
    let dir = common::new_tempdir()?;

    let output = common::render_module("docker")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn folder_with_dockerfile() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("Dockerfile"))?;

    let output = common::render_module("docker")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Blue.bold().paint("🐳 v2.6.3"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn folder_with_dockercompose_file() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("docker-compose.yml"))?;

    let output = common::render_module("docker")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Blue.bold().paint("🐳 v2.6.3"));
    assert_eq!(expected, actual);
    Ok(())
}
