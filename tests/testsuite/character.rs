use ansi_term::Color;
use std::io;

use crate::common::{self, TestCommand};

#[test]
fn char_module_success_status() -> io::Result<()> {
    let expected = format!("{} ", Color::Green.bold().paint("❯"));

    // Status code 0
    let output = common::render_module("character")
        .arg("--status=0")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    // No status code
    let output = common::render_module("character").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn char_module_failure_status() -> io::Result<()> {
    let expected = format!("{} ", Color::Red.bold().paint("❯"));

    let exit_values = ["1", "54321", "-5000"];

    for status in exit_values.iter() {
        let arg = format!("--status={}", status);
        let output = common::render_module("character").arg(arg).output()?;
        let actual = String::from_utf8(output.stdout).unwrap();
        assert_eq!(expected, actual);
    }

    Ok(())
}

#[test]
fn char_module_symbolyes_status() -> io::Result<()> {
    let expected_fail = format!("{} ", Color::Red.bold().paint("✖"));
    let expected_success = format!("{} ", Color::Green.bold().paint("❯"));

    let exit_values = ["1", "54321", "-5000"];

    // Test failure values
    for status in exit_values.iter() {
        let arg = format!("--status={}", status);
        let output = common::render_module("character")
            .use_config(toml::toml! {
                [character]
                use_symbol_for_status = true
            })
            .arg(arg)
            .output()?;
        let actual = String::from_utf8(output.stdout).unwrap();
        assert_eq!(expected_fail, actual);
    }

    // Test success
    let output = common::render_module("character")
        .use_config(toml::toml! {
            [character]
            use_symbol_for_status = true
        })
        .arg("--status=0")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected_success, actual);

    Ok(())
}

#[test]
fn char_module_vicmd_keymap() -> io::Result<()> {
    let expected_vicmd = format!("{} ", Color::Green.bold().paint("❮"));
    let expected_specified = format!("{} ", Color::Green.bold().paint("N"));
    let expected_other = format!("{} ", Color::Green.bold().paint("❯"));

    // zle keymap is vicmd
    let output = common::render_module("character")
        .arg("--keymap=vicmd")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected_vicmd, actual);

    // specified vicmd character
    let output = common::render_module("character")
        .use_config(toml::toml! {
            [character]
            vicmd_symbol = "N"
        })
        .arg("--keymap=vicmd")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected_specified, actual);

    // zle keymap is other
    let output = common::render_module("character")
        .arg("--keymap=visual")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected_other, actual);

    Ok(())
}
