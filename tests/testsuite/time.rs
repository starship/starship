use ansi_term::Color;
use std::fs;
use std::io;
use std::path::Path;
use tempfile::TempDir;

use crate::common::{self, TestCommand};

#[test]
fn config_enabled() -> io::Result<()> {
    let output = common::render_module("time")
        .use_config(toml::toml! {
            [time]
            disabled = false
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn config_blank() -> io::Result<()> {
    let output = common::render_module("time").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}
