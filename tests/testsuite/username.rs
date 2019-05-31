use ansi_term::Color;
use std::io;

use crate::common;

// TODO: Add tests for if root user (UID == 0)
// Requires mocking

#[test]
fn no_username_shown() -> io::Result<()> {
    let expected = "";

    // No environment variables
    let output = common::render_module("username").env_clear().output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    // LOGNAME == USER
    let output = common::render_module("username")
        .env_clear()
        .env("LOGNAME", "astronaut")
        .env("USER", "astronaut")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    // SSH connection w/o username
    let output = common::render_module("username")
        .env_clear()
        .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn current_user_not_logname() -> io::Result<()> {
    let output = common::render_module("username")
        .env_clear()
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
        .env_clear()
        .env("USER", "astronaut")
        .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("astronaut"));
    assert_eq!(expected, actual);
    Ok(())
}
