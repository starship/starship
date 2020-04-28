use std::io;

use crate::common;

#[test]
fn unknown_module_name() -> io::Result<()> {
    let unknown_module_name = "some_random_name";
    let output = common::render_module(unknown_module_name).output()?;
    let actual_stdout = String::from_utf8(output.stdout).unwrap();
    let actual_stderr = String::from_utf8(output.stderr).unwrap();
    let expected_stdout = "";
    let expected_stderr = format!(
        "Error: Unknown module {}. Use starship module --list to list out all supported modules.\n",
        unknown_module_name
    );
    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(expected_stderr, actual_stderr);
    Ok(())
}

#[test]
fn known_module_name() -> io::Result<()> {
    let output = common::render_module("line_break").output()?;
    let actual_stdout = String::from_utf8(output.stdout).unwrap();
    let actual_stderr = String::from_utf8(output.stderr).unwrap();
    let expected_stdout = "\n";
    let expected_stderr = "";
    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(expected_stderr, actual_stderr);
    Ok(())
}
