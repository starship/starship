use ansi_term::{Color, Style};
use std::io;

use crate::common;
use crate::common::TestCommand;

// TODO: test where hostname contains invalid UTF, triggering invalid hostname pattern

#[test]
fn ssh_only_false() -> io::Result<()> {
    let output = common::render_module("hostname")
        .env_clear()
        .use_config(toml::toml! {
            [hostname]
            ssh_only = false
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!("on {} ", style().paint(hostname()));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn no_ssh() -> io::Result<()> {
    let output = common::render_module("hostname")
        .env_clear()
        .use_config(toml::toml! {
            [hostname]
            ssh_only = true
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!("", actual);
    Ok(())
}

#[test]
fn ssh() -> io::Result<()> {
    let output = common::render_module("hostname")
        .env_clear()
        .use_config(toml::toml! {
            [hostname]
            ssh_only = true
        })
        .env("SSH_CONNECTION", "something")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!("on {} ", style().paint(hostname()));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn disabled() -> io::Result<()> {
    let output = common::render_module("hostname")
        .env_clear()
        .use_config(toml::toml! {
            [hostname]
            disabled = true
        })
        .env("SSH_CONNECTION", "something")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!("", actual);
    Ok(())
}

#[test]
fn prefix() -> io::Result<()> {
    let output = common::render_module("hostname")
        .env_clear()
        .use_config(toml::toml! {
            [hostname]
            ssh_only = false
            prefix = "<"
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!("on {} ", style().paint(format!("<{}", hostname())));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn suffix() -> io::Result<()> {
    let output = common::render_module("hostname")
        .env_clear()
        .use_config(toml::toml! {
            [hostname]
            ssh_only = false
            suffix = ">"
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!("on {} ", style().paint(format!("{}>", hostname())));
    assert_eq!(actual, expected);
    Ok(())
}

fn hostname() -> String {
    gethostname::gethostname().into_string().unwrap()
}

fn style() -> Style {
    Color::Green.bold().dimmed()
}
