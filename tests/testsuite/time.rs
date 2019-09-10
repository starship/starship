use ansi_term::Color;
use std::fs;
use std::io;
use std::path::Path;
use tempfile::TempDir;

use crate::common::{self, TestCommand};

/* Note: tests in this crate cannot rely on the actual time displayed by
the module, since that is dependent on the time inside the test environment,
which we cannot control.

However, we *can* test certain things here, such as the fact that the module
should not display when disabled, should display *something* when enabled,
and should have the correct prefixes and suffixes in a given config */

#[test]
#[ignore]
fn config_enabled() -> io::Result<()> {
    let output = common::render_module("time")
        .use_config(toml::toml! {
            [time]
            disabled = false
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    // We can't test what it actually is...but we can assert it's not blank
    assert!(actual.len() > 0);
    Ok(())
}

#[test]
#[ignore]
fn config_blank() -> io::Result<()> {
    let output = common::render_module("time").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
fn config_check_prefix_and_suffix() -> io::Result<()> {
    let output = common::render_module("time")
        .use_config(toml::toml! {
            [time]
            disabled = false
            format = "[%T]"
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    // This is the prefix with "at ", the color code, then the prefix char [
    let col_prefix = format!("at {}{}[", '\u{1b}', "[1;33m");

    // This is the suffix with suffix char ']', then color codes, then a space
    let col_suffix = format!("]{}{} ", '\u{1b}', "[0m");

    assert!(actual.starts_with(&col_prefix));
    assert!(actual.ends_with(&col_suffix));
    Ok(())
}
