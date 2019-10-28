use ansi_term::Color;
use std::io;

use crate::common;

#[test]
fn env_set() -> io::Result<()> {
    let output = common::render_module("singularity")
        .env_clear()
        .env("SINGULARITY_NAME", "centos.img")
        .output()?;

    let expected = format!("{} ", Color::Blue.bold().dimmed().paint("[centos.img]"));
    let actual = String::from_utf8(output.stdout).unwrap();

    assert_eq!(expected, actual);
    Ok(())
}
