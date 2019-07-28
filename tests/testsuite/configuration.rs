use ansi_term::Color;
use std::io;

use crate::common::{self, TestCommand};

#[test]
fn char_symbol_configuration() -> io::Result<()> {
    let expected = format!("{} ", Color::Green.bold().paint("❯"));

    let output = common::render_module("character")
        .use_config(toml::toml! {
            [character]
            symbol = "❯"
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn disabled_module() -> io::Result<()> {
    let output = common::render_module("package")
        .use_config(toml::toml! {
            [package]
            disabled = true
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!("", actual);

    Ok(())
}

#[test]
fn add_newline_configuration() -> io::Result<()> {
    // Start prompt with newline
    let default_output = common::render_prompt().output()?;
    let actual = String::from_utf8(default_output.stdout).unwrap();
    let expected = actual.trim_start();
    assert_ne!(actual, expected);

    // Start prompt without newline
    let output = common::render_prompt()
        .use_config(toml::toml! {
            add_newline = false
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = actual.trim_start();
    assert_eq!(expected, actual);

    Ok(())
}
