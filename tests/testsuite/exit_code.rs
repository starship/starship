use ansi_term::Color;
use std::io;

use crate::common::{self, TestCommand};

#[test]
fn success_status() -> io::Result<()> {
    let expected = "";

    // Status code 0
    let output = common::render_module("exit_code")
        .arg("--status=0")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    // No status code
    let output = common::render_module("exit_code").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn failure_status() -> io::Result<()> {
    let exit_values = ["1", "2", "130"];

    for status in exit_values.iter() {
        let expected = format!("{} ", Color::Red.bold().paint(format!("✖{}", status)));
        let arg = format!("--status={}", status);
        let output = common::render_module("exit_code").arg(arg).output()?;
        let actual = String::from_utf8(output.stdout).unwrap();
        assert_eq!(expected, actual);
    }

    Ok(())
}
