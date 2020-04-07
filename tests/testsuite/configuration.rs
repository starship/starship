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
