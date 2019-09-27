use ansi_term::Color;
use std::io;

use crate::common::{self, TestCommand};

#[test]
fn config_blank_duration_1s() -> io::Result<()> {
    let output = common::render_module("cmd_duration").arg("--cmd-duration=1").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn config_blank_duration_5s() -> io::Result<()> {
    let output = common::render_module("cmd_duration").arg("--cmd-duration=5").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("{} ", Color::Yellow.bold().paint("took 5s"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn config_5s_duration_3s() -> io::Result<()> {
    let output = common::render_module("cmd_duration")
        .use_config(toml::toml! {
            [cmd_duration]
            min_time = 5
        })
        .arg("--cmd-duration=3")
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
            min_time = 5
        })
        .arg("--cmd-duration=10")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("{} ", Color::Yellow.bold().paint("took 10s"));
    assert_eq!(expected, actual);
    Ok(())
}
