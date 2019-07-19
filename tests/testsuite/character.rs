use ansi_term::Color;
use std::io;

use crate::common;

#[test]
fn char_module_success_status() -> io::Result<()> {
    let expected = format!("{} ", Color::Green.bold().paint("➜"));

    // Status code 0
    let output = common::render_module("character").arg("--status=0").output()?;
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
    let expected = format!("{} ", Color::Red.bold().paint("➜"));

    // Error status code 1
    let output = common::render_module("character").arg("--status=1").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    // Random non-zero status code
    let output = common::render_module("character")
        .arg("--status=54321")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    // Negative status code!?
    let output = common::render_module("character")
        .arg("--status=-5000")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}
