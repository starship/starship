use ansi_term::Color;
use std::io;

use crate::common::{self, TestCommand};

#[test]
fn config_blank_duration_1s() -> io::Result<()> {
    let output = common::render_module("cmd_duration")
        .arg("--cmd-duration=1000")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn config_blank_duration_5s() -> io::Result<()> {
    let output = common::render_module("cmd_duration")
        .arg("--cmd-duration=5000")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("took {} ", Color::Yellow.bold().paint("5s"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn config_5s_duration_3s() -> io::Result<()> {
    let output = common::render_module("cmd_duration")
        .use_config(toml::toml! {
            [cmd_duration]
            min_time = 5000
        })
        .arg("--cmd-duration=3000")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn config_5s_duration_10s() -> io::Result<()> {
    let output = common::render_module("cmd_duration")
        .use_config(toml::toml! {
            [cmd_duration]
            min_time = 5000
        })
        .arg("--cmd-duration=10000")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("took {} ", Color::Yellow.bold().paint("10s"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn config_1s_duration_prefix_underwent() -> io::Result<()> {
    let output = common::render_module("cmd_duration")
        .use_config(toml::toml! {
            [cmd_duration]
            prefix = "underwent "
        })
        .arg("--cmd-duration=1000")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn config_5s_duration_prefix_underwent() -> io::Result<()> {
    let output = common::render_module("cmd_duration")
        .use_config(toml::toml! {
            [cmd_duration]
            prefix = "underwent "
        })
        .arg("--cmd-duration=5000")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("underwent {} ", Color::Yellow.bold().paint("5s"));
    assert_eq!(expected, actual);
    Ok(())
}
