use ansi_term::Color;
use std::io;

use crate::common::{self, TestCommand};

// TODO: Add tests for if root user (UID == 0)
// Requires mocking

#[test]
#[cfg(not(target_os = "windows"))]
fn no_env_variables() -> io::Result<()> {
    let output = common::render_module("username").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!("", actual);
    Ok(())
}

#[test]
#[cfg(not(target_os = "windows"))]
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
#[cfg(not(target_os = "windows"))]
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
#[cfg(not(target_os = "windows"))]
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
#[cfg(not(target_os = "windows"))]
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

#[test]
#[cfg(target_os = "windows")]
fn ssh_connection() -> io::Result<()> {
    let output = common::render_module("username")
        .env("USERNAME", "astronaut")
        .env("SSH_CONNECTION", "192.168.223.17 36673 192.168.223.229 22")
        .use_config(toml::toml! {
            [username]
            style_root	= "bold red"
            style_user	= "bold red"})
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("astronaut"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[cfg(not(target_os = "windows"))]
fn show_always() -> io::Result<()> {
    let output = common::render_module("username")
        .env("USER", "astronaut")
        .use_config(toml::toml! {
        [username]
        show_always = true})
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Yellow.bold().paint("astronaut"));

    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
fn show_always() -> io::Result<()> {
    let output = common::render_module("username")
        .env("USERNAME", "astronaut")
        .use_config(toml::toml! {
        [username]
        show_always = true
        style_root	= "bold red"
        style_user	= "bold red"})
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("astronaut"));

    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
fn current_user_local_account() -> io::Result<()> {
    let output = common::render_module("username")
        .env("USERNAME", "astronaut")
        .env("USERDOMAIN", "myhost")
        .env("COMPUTERNAME", "myhost")
        .use_config(toml::toml! {
        [username]
        show_always = true
        style_root	= "bold red"
        style_user	= "bold red"})
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("astronaut"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[cfg(target_os = "windows")]
fn current_user_domain_account() -> io::Result<()> {
    let output = common::render_module("username")
        .env("USERNAME", "astronaut")
        .env("USERDOMAIN", "nasa")
        .env("COMPUTERNAME", "myhost")
        .use_config(toml::toml! {
        [username]
        show_always = true
        style_root	= "bold red"
        style_user	= "bold red"})
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("nasa\\astronaut"));
    assert_eq!(expected, actual);
    Ok(())
}
