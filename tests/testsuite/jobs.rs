use ansi_term::Color;
use std::fs;
use std::io;
use std::path::Path;
use tempfile::TempDir;

use crate::common::{self, TestCommand};

#[test]
fn config_blank_job_0() -> io::Result<()> {
    let output = common::render_module("jobs").arg("--jobs=0").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn config_blank_job_1() -> io::Result<()> {
    let output = common::render_module("jobs").arg("--jobs=1").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("{} ", Color::Blue.bold().paint("✦ "));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn config_blank_job_2() -> io::Result<()> {
    let output = common::render_module("jobs").arg("--jobs=2").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("{} ", Color::Blue.bold().paint("✦ 2"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn config_2_job_2() -> io::Result<()> {
    let output = common::render_module("jobs")
        .use_config(toml::toml! {
            [jobs]
            threshold = 2
        })
        .arg("--jobs=2")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("{} ", Color::Blue.bold().paint("✦ "));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn config_2_job_3() -> io::Result<()> {
    let output = common::render_module("jobs")
        .use_config(toml::toml! {
            [jobs]
            threshold = 2
        })
        .arg("--jobs=3")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("{} ", Color::Blue.bold().paint("✦ 3"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn config_disabled() -> io::Result<()> {
    let output = common::render_module("jobs")
        .use_config(toml::toml! {
            [jobs]
            disabled = true
        })
        .arg("--jobs=1")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}
