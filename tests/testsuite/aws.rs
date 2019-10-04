use ansi_term::Color;
use std::io;

use crate::common;

#[test]
fn no_region_set() -> io::Result<()> {
    let output = common::render_module("aws").env_clear().output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn region_set() -> io::Result<()> {
    let output = common::render_module("aws")
        .env_clear()
        .env("AWS_REGION", "ap-northeast-2")
        .output()?;
    let expected = format!("on {} ", Color::Yellow.bold().paint("☁️ ap-northeast-2"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn default_region_set() -> io::Result<()> {
    let output = common::render_module("aws")
        .env_clear()
        .env("AWS_REGION", "ap-northeast-2")
        .env("AWS_DEFAULT_REGION", "ap-northeast-1")
        .output()?;
    let expected = format!("on {} ", Color::Yellow.bold().paint("☁️ ap-northeast-1"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn profile_set() -> io::Result<()> {
    let output = common::render_module("aws")
        .env_clear()
        .env("AWS_PROFILE", "astronauts")
        .output()?;
    let expected = format!("on {} ", Color::Yellow.bold().paint("☁️ astronauts"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}
