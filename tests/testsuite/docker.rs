use ansi_term::Color;
use std::fs::{self, File};
use std::io::{self, Write};

use crate::common;

#[test]
fn folder_without_docker_files() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let output = common::render_module("docker")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    dir.close()
}

#[test]
fn folder_with_dockerfile() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("Dockerfile"))?;

    let output = common::render_module("docker")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("{} ", Color::Blue.bold().paint("🐳"));
    assert_eq!(expected, actual);
    dir.close()
}

#[test]
fn folder_with_docker_compose_file() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("docker-compose.yml"))?;

    let output = common::render_module("docker")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("{} ", Color::Blue.bold().paint("🐳"));
    assert_eq!(expected, actual);
    dir.close()
}
