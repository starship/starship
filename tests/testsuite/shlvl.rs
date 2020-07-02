use ansi_term::{Color, Style};
use std::io;

use crate::common;
use crate::common::TestCommand;

const SHLVL_ENV_VAR: &str = "SHLVL";

fn style() -> Style {
    // default style
    Color::Yellow.bold()
}

#[test]
fn empty_config() -> io::Result<()> {
    let output = common::render_module("shlvl")
        .env_clear()
        .use_config(toml::toml! {
            [shlvl]
        })
        .env(SHLVL_ENV_VAR, "2")
        .output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn enabled() -> io::Result<()> {
    let output = common::render_module("shlvl")
        .env_clear()
        .use_config(toml::toml! {
            [shlvl]
            disabled = false
        })
        .env(SHLVL_ENV_VAR, "2")
        .output()?;
    let expected = format!("{} ", style().paint("↕️  2"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn no_level() -> io::Result<()> {
    let output = common::render_module("shlvl")
        .env_clear()
        .use_config(toml::toml! {
            [shlvl]
            disabled = false
        })
        .output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn enabled_config_level_1() -> io::Result<()> {
    let output = common::render_module("shlvl")
        .env_clear()
        .use_config(toml::toml! {
            [shlvl]
            disabled = false
        })
        .env(SHLVL_ENV_VAR, "1")
        .output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn lower_threshold() -> io::Result<()> {
    let output = common::render_module("shlvl")
        .env_clear()
        .use_config(toml::toml! {
            [shlvl]
            threshold = 1
            disabled = false
        })
        .env(SHLVL_ENV_VAR, "1")
        .output()?;
    let expected = format!("{} ", style().paint("↕️  1"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn higher_threshold() -> io::Result<()> {
    let output = common::render_module("shlvl")
        .env_clear()
        .use_config(toml::toml! {
            [shlvl]
            threshold = 3
            disabled = false
        })
        .env(SHLVL_ENV_VAR, "1")
        .output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn custom_style() -> io::Result<()> {
    let output = common::render_module("shlvl")
        .env_clear()
        .use_config(toml::toml! {
            [shlvl]
            style = "Red Underline"
            disabled = false
        })
        .env(SHLVL_ENV_VAR, "2")
        .output()?;
    let expected = format!("{} ", Color::Red.underline().paint("↕️  2"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn custom_symbol() -> io::Result<()> {
    let output = common::render_module("shlvl")
        .env_clear()
        .use_config(toml::toml! {
            [shlvl]
            symbol = "shlvl is "
            disabled = false
        })
        .env(SHLVL_ENV_VAR, "2")
        .output()?;
    let expected = format!("{} ", style().paint("shlvl is 2"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn prefix_and_suffix() -> io::Result<()> {
    let output = common::render_module("shlvl")
        .env_clear()
        .use_config(toml::toml! {
            [shlvl]
            prefix = "shlvl "
            suffix = " level(s) "
            disabled = false
        })
        .env(SHLVL_ENV_VAR, "2")
        .output()?;
    let expected = format!("shlvl {} level(s) ", style().paint("↕️  2"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}
