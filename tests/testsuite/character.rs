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
    let expected_vicmd = "❮";
    // TODO make this less... well, stupid when ANSI escapes can be mocked out
    let expected_specified = "I HIGHLY DOUBT THIS WILL SHOW UP IN OTHER OUTPUT";
    let expected_other = "❯";

    // zle keymap is vicmd
    let output = common::render_module("character")
        .env("STARSHIP_SHELL", "zsh")
        .arg("--keymap=vicmd")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert!(actual.contains(&expected_vicmd));

    // specified vicmd character
    let output = common::render_module("character")
        .use_config(toml::toml! {
            [character]
            vicmd_symbol = "I HIGHLY DOUBT THIS WILL SHOW UP IN OTHER OUTPUT"
        })
        .env("STARSHIP_SHELL", "zsh")
        .arg("--keymap=vicmd")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert!(actual.contains(&expected_specified));

    // zle keymap is other
    let output = common::render_module("character")
        .env("STARSHIP_SHELL", "zsh")
        .arg("--keymap=visual")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert!(actual.contains(&expected_other));

    Ok(())
}
