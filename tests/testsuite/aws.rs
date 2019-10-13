use ansi_term::Color;
use std::io;

use crate::common;

#[test]
fn no_profile_set() -> io::Result<()> {
    let output = common::render_module("aws").env_clear().output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn profile_set() -> io::Result<()> {
    let output = common::render_module("aws")
        .env_clear()
        .env("AWS_PROFILE", "astronauts")
        .output()?;
    let expected = format!("on {} ", Color::Yellow.bold().paint("☁️  astronauts"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}
