use ansi_term::Color;
use std::io;

use crate::common::{self, TestCommand};

#[test]
fn success_status_disabled() -> io::Result<()> {
    let expected = String::new();

    // Status code 0
    let output = common::render_module("exit_code")
        .use_config(toml::toml! {
            [exit_code]
            disabled = true
        })
        .arg("--status=0")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    // No status code
    let output = common::render_module("exit_code")
        .use_config(toml::toml! {
            [exit_code]
            disabled = true
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn success_status_enabled() -> io::Result<()> {
    let expected = String::new();

    // Status code 0, implicitly enabled
    let output = common::render_module("exit_code")
        .arg("--status=0")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    // No status code, implicitly enabled
    let output = common::render_module("exit_code").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    // Status code 0, explicitly enabled
    let output = common::render_module("exit_code")
        .use_config(toml::toml! {
            [exit_code]
            disabled = false
        })
        .arg("--status=0")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    // No status code, explicitly enabled
    let output = common::render_module("exit_code")
        .use_config(toml::toml! {
            [exit_code]
            disabled = false
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn failure_status_disabled() -> io::Result<()> {
    let expected = String::new();
    let exit_values = ["1", "54321", "-5000"];

    for status in exit_values.iter() {
        let arg = format!("--status={}", status);
        let output = common::render_module("exit_code")
            .use_config(toml::toml! {
                [exit_code]
                disabled = true
            })
            .arg(arg)
            .output()?;
        let actual = String::from_utf8(output.stdout).unwrap();
        assert_eq!(expected, actual);
    }

    Ok(())
}

#[test]
fn failure_status_enabled() -> io::Result<()> {
    let exit_values = ["1", "54321", "-5000"];

    for status in exit_values.iter() {
        let expected = format!("{} ", Color::Red.bold().paint(*status));

        // Implicitly enabled
        let arg = format!("--status={}", status);
        let output = common::render_module("exit_code").arg(arg).output()?;
        let actual = String::from_utf8(output.stdout).unwrap();
        assert_eq!(expected, actual);

        // Explicitly enabled
        let arg = format!("--status={}", status);
        let output = common::render_module("exit_code")
            .use_config(toml::toml! {
                [exit_code]
                disabled = false
            })
            .arg(arg)
            .output()?;
        let actual = String::from_utf8(output.stdout).unwrap();
        assert_eq!(expected, actual);
    }

    Ok(())
}
