use ansi_term::Color;
use std::io;

use crate::common::{self, TestCommand};

#[test]
fn char_symbol_configuration() -> io::Result<()> {
    let expected = format!("{} ", Color::Green.bold().paint("❯"));

    let output = common::render_module("char")
        .use_config(toml::toml! {
            [char]
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
