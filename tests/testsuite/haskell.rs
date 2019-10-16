use std::fs::File;
use std::io;

use ansi_term::Color;
use tempfile;

use crate::common;

#[test]
#[ignore]
fn folder_with_stack_yaml() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("stack.yaml"))?.sync_all()?;

    let output = common::render_module("haskell")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("λ v8.6.5"));
    assert_eq!(expected, actual);
    Ok(())
}
