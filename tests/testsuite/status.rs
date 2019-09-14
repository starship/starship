use ansi_term::Color;
use std::io;

use crate::common::{self, TestCommand};

#[test]
fn success_exit() -> io::Result<()> {
    let expected = "";

    let output = common::render_module("status").arg("--status=0").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    let output = common::render_module("status").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn error_exit() -> io::Result<()> {
    let expected = format!("{} ", Color::Red.paint("1"));

    let output = common::render_module("status").arg("--status=1").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn success_pipeline() -> io::Result<()> {
    let expected = format!("{} ", Color::White.dimmed().paint("(1 0)"));

    let output = common::render_module("status")
        .arg("--status=1 0")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn error_pipeline() -> io::Result<()> {
    let expected = format!("{} ", Color::Red.paint("(0 1)"));

    let output = common::render_module("status")
        .arg("--status=0 1")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn no_pipeline() -> io::Result<()> {
    let config = toml::toml! {
        [status]
        no_pipeline = true
    };

    let expected = "";

    let output = common::render_module("status")
        .use_config(config.clone())
        .arg("--status=1 0")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    let expected = format!("{} ", Color::Red.paint("1"));

    let output = common::render_module("status")
        .use_config(config)
        .arg("--status=0 1")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn show_success() -> io::Result<()> {
    let expected = format!("{} ", Color::White.dimmed().paint("0"));

    let output = common::render_module("status")
        .use_config(toml::toml! {
            [status]
            show_success = true
        })
        .arg("--status=0")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn show_pipeline_always() -> io::Result<()> {
    let config = toml::toml! {
        [status]
        show_pipeline_always = false
    };
    let expected = "";

    let output = common::render_module("status")
        .use_config(config.clone())
        .arg("--status=1 0")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    let expected = format!("{} ", Color::Red.paint("(0 1)"));

    let output = common::render_module("status")
        .use_config(config)
        .arg("--status=0 1")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn use_symbols() -> io::Result<()> {
    let config = toml::toml! {
        [status]
        use_symbols = true
    };

    let expected = format!("{} ", Color::White.dimmed().paint("(✖ ✔)"));

    let output = common::render_module("status")
        .use_config(config.clone())
        .arg("--status=1 0")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    let expected = format!("{} ", Color::Red.paint("(✔ ✖)"));

    let output = common::render_module("status")
        .use_config(config)
        .arg("--status=0 1")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}
