use std::fs::File;
use std::io;

use ansi_term::Color;

use crate::common;

#[test]
#[ignore]
fn folder_with_pom() -> io::Result<()> {
    let dir = common::new_tempdir()?;
    File::create(dir.path().join("pom.xml"))?;

    let output = common::render_module("java")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "via {} ",
        Color::RGB(166, 42, 42).bold().paint("☕ v12.0.2")
    );
    assert_eq!(expected, actual);
    Ok(())
}
