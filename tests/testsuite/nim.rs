use std::fs::File;
use std::io;

use ansi_term::Color;

use crate::common;

#[test]
fn folder_with_nim_version() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("nim_test.nimble"))?;

    let output = common::render_module("nim")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("👑 v0.19.4"));
    assert_eq!(expected, actual);
    Ok(())
}
