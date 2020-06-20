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

    let expected = format!("via {} ", Color::Blue.bold().paint("❄️  pure"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn impure_shell() -> io::Result<()> {
    let output = common::render_module("nix_shell")
        .env("IN_NIX_SHELL", "impure")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Blue.bold().paint("❄️  impure"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn pure_shell_name() -> io::Result<()> {
    let output = common::render_module("nix_shell")
        .env("IN_NIX_SHELL", "pure")
        .env("name", "starship")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Blue.bold().paint("❄️  pure (starship)"));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn impure_shell_name() -> io::Result<()> {
    let output = common::render_module("nix_shell")
        .env("IN_NIX_SHELL", "impure")
        .env("name", "starship")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Blue.bold().paint("❄️  impure (starship)"));
    assert_eq!(expected, actual);
    Ok(())
}
