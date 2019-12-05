use ansi_term::{Color, Style};
use std::io;

use crate::common;
use crate::common::TestCommand;

const TEST_VAR_VALUE: &str = "astronauts";

#[test]
fn empty_config() -> io::Result<()> {
    let output = common::render_module("env_var")
        .env_clear()
        .use_config(toml::toml! {
            [env_var]
        })
        .output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn defined_variable() -> io::Result<()> {
    let output = common::render_module("env_var")
        .env_clear()
        .use_config(toml::toml! {
            [env_var]
            variable = "TEST_VAR"
        })
        .env("TEST_VAR", TEST_VAR_VALUE)
        .output()?;
    let expected = format!("with {} ", style().paint(TEST_VAR_VALUE));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn undefined_variable() -> io::Result<()> {
    let output = common::render_module("env_var")
        .env_clear()
        .use_config(toml::toml! {
            [env_var]
            variable = "TEST_VAR"
        })
        .output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn default_has_no_effect() -> io::Result<()> {
    let output = common::render_module("env_var")
        .env_clear()
        .use_config(toml::toml! {
            [env_var]
            variable = "TEST_VAR"
            default = "N/A"
        })
        .env("TEST_VAR", TEST_VAR_VALUE)
        .output()?;
    let expected = format!("with {} ", style().paint(TEST_VAR_VALUE));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn default_takes_effect() -> io::Result<()> {
    let output = common::render_module("env_var")
        .env_clear()
        .use_config(toml::toml! {
            [env_var]
            variable = "UNDEFINED_TEST_VAR"
            default = "N/A"
        })
        .output()?;
    let expected = format!("with {} ", style().paint("N/A"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn format() -> io::Result<()> {
    let output = common::render_module("env_var")
        .env_clear()
        .use_config(toml::toml! {
            [env_var]
            format = "with ${styled?value=■ &style=black bold dimmed}${variable?style=black bold dimmed} "
            variable = "TEST_VAR"
        })
        .env("TEST_VAR", TEST_VAR_VALUE)
        .output()?;
    let expected = format!("with {} ", style().paint(format!("■ {}", TEST_VAR_VALUE)));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

fn style() -> Style {
    // default style
    Color::Black.bold().dimmed()
}
