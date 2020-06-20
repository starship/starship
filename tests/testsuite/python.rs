use std::fs::File;
use std::io;

use crate::common;

// TODO - These tests should be moved into the python module when we have sorted out mocking of env
// vars.

#[test]
fn with_virtual_env() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    File::create(dir.path().join("main.py"))?.sync_all()?;
    let output = common::render_module("python")
        .env("VIRTUAL_ENV", "/foo/bar/my_venv")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    assert!(actual.contains("my_venv"));
    dir.close()
}

#[test]
fn with_active_venv() -> io::Result<()> {
    let dir = tempfile::tempdir()?;

    let output = common::render_module("python")
        .env("VIRTUAL_ENV", "/foo/bar/my_venv")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    assert!(actual.contains("my_venv"));
    dir.close()
}
