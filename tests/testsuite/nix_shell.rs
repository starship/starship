use ansi_term::Color;
use std::io;

use crate::common;

#[test]
fn no_env_variables() -> io::Result<()> {
    let output = common::render_module("nix_shell").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!("", actual);
    Ok(())
}

#[test]
fn invalid_env_variables() -> io::Result<()> {
    let output = common::render_module("nix_shell")
        .env("IN_NIX_SHELL", "something_wrong")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!("", actual);
    Ok(())
}

#[test]
fn pure_shell() -> io::Result<()> {
    let output = common::render_module("nix_shell")
        .env("IN_NIX_SHELL", "pure")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("pure"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn impure_shell() -> io::Result<()> {
    let output = common::render_module("nix_shell")
        .env("IN_NIX_SHELL", "impure")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("impure"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn lorri_shell() -> io::Result<()> {
    let output = common::render_module("nix_shell")
        .env("IN_NIX_SHELL", "1")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("impure"));
    assert_eq!(expected, actual);
    Ok(())
}
