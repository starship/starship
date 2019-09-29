use ansi_term::Color;
use std::io;

use crate::common::{self, TestCommand};

#[test]
fn success_exit() -> io::Result<()> {
    let expected = "";

    let output = common::render_module("status")
        .args(&["--status=0", "--pipestatus=0"])
        .output()?;
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

    let output = common::render_module("status")
        .args(&["--status=1", "--pipestatus=1"])
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn success_pipeline() -> io::Result<()> {
    let expected = format!("{} ", Color::White.dimmed().paint("(1 0)"));

    let output = common::render_module("status")
        .args(&["--status=0", "--pipestatus=1 0"])
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn error_pipeline() -> io::Result<()> {
    let expected = format!("{} ", Color::Red.paint("(0 1)"));

    let output = common::render_module("status")
        .args(&["--status=1", "--pipestatus=0 1"])
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn full_pipeline() -> io::Result<()> {
    let config = toml::toml! {
        [status]
        display_mode = "always"
        simple_pipeline = false
    };

    let expected = format!("{} ", Color::White.dimmed().paint("(0 0 0)"));

    let output = common::render_module("status")
        .use_config(config.clone())
        .args(&["--status=0", "--pipestatus=0 0 0"])
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    let expected = format!("{} ", Color::Red.paint("(0 1)"));

    let output = common::render_module("status")
        .use_config(config)
        .args(&["--status=1", "--pipestatus=0 1"])
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn show_always() -> io::Result<()> {
    let config = toml::toml! {
        [status]
        display_mode = "always"
    };

    let expected = format!("{} ", Color::White.dimmed().paint("0"));

    let output = common::render_module("status")
        .use_config(config.clone())
        .args(&["--status=0", "--pipestatus=0"])
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    let expected = format!("{} ", Color::White.dimmed().paint("0"));

    let output = common::render_module("status")
        .use_config(config)
        .args(&["--status=0", "--pipestatus=0 0 0"])
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn only_error() -> io::Result<()> {
    let config = toml::toml! {
        [status]
        display_mode = "error"
    };

    let tests = [
        ("".to_string(), ["0", "0"]),
        ("".to_string(), ["0", "1 0"]),
        (format!("{} ", Color::Red.paint("1")), ["1", "1"]),
        (format!("{} ", Color::Red.paint("(0 1)")), ["1", "0 1"]),
    ];

    for (expected, args) in tests.iter() {
        let args = [
            format!("--status={}", args[0]),
            format!("--pipestatus={}", args[1]),
        ];
        let output = common::render_module("status")
            .use_config(config.clone())
            .args(&args)
            .output()?;
        let actual = String::from_utf8(output.stdout).unwrap();
        assert_eq!(*expected, actual);
    }

    Ok(())
}

#[test]
fn any_error() -> io::Result<()> {
    let expected = format!("{} ", Color::White.dimmed().paint("(1 0)"));

    let output = common::render_module("status")
        .args(&["--status=0", "--pipestatus=1 0"])
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
        .args(&["--status=0", "--pipestatus=1 0"])
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    let expected = format!("{} ", Color::Red.paint("(✔ ✖)"));

    let output = common::render_module("status")
        .use_config(config)
        .args(&["--status=1", "--pipestatus=0 1"])
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn mismatch() -> io::Result<()> {
    let expected = format!("{} ", Color::White.dimmed().paint("1 [0]"));

    let output = common::render_module("status")
        .args(&["--status=0", "--pipestatus=1"])
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    let expected = format!("{} ", Color::Red.paint("0 [1]"));

    let output = common::render_module("status")
        .args(&["--status=1", "--pipestatus=0"])
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}
