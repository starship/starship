use ansi_term::Color;
use std::fs::File;
use std::io;

use crate::common;

#[test]
fn folder_without_elixir_files() -> io::Result<()> {
    let dir = common::new_tempdir()?;

    let output = common::render_module("elixir")
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
fn folder_with_mixfile() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("mix.exs"))?;

    let output = common::render_module("elixir")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Purple.bold().paint("🧪 v1.9.1 OTP/21"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn folder_with_ex_file() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("any.ex"))?;

    let output = common::render_module("elixir")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Purple.bold().paint("🧪 v1.9.1 OTP/21"));
    assert_eq!(expected, actual);
    Ok(())
}
