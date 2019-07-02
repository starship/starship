use ansi_term::Color;
use std::io;

use crate::common;

// TODO: Add tests for if root user (UID == 0)
// Requires mocking

#[test]
fn no_env_variables() -> io::Result<()> {
    let output = common::render_module("username").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!("", actual);
    Ok(())
}

#[test]
fn logname_equals_user() -> io::Result<()> {
    let output = common::render_module("username")
        .env("LOGNAME", "astronaut")
        .env("USER", "astronaut")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!("", actual);
    Ok(())
}

#[test]
fn ssh_wo_username() -> io::Result<()> {
    // SSH connection w/o username
    let output = common::render_module("username")
        .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!("", actual);
    Ok(())
}

#[test]
fn current_user_not_logname() -> io::Result<()> {
    let output = common::render_module("username")
        .env("LOGNAME", "astronaut")
        .env("USER", "cosmonaut")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("cosmonaut"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn ssh_connection() -> io::Result<()> {
    let output = common::render_module("username")
        .env("USER", "astronaut")
        .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("astronaut"));
    assert_eq!(expected, actual);
    Ok(())
}
