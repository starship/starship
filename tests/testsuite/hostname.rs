use ansi_term::{Color, Style};
use std::io;

use crate::{common, common::TestCommand};

#[test]
fn ssh_only_false() -> io::Result<()> {
    let hostname = match get_hostname() {
        Some(h) => h,
        None => return hostname_not_tested(),
    };
    let output = common::render_module("hostname")
        .env_clear()
        .use_config(toml::toml! {
            [hostname]
            ssh_only = false
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!("on {} ", style().paint(hostname));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn no_ssh() -> io::Result<()> {
    let output = common::render_module("hostname")
        .env_clear()
        .use_config(toml::toml! {
            [hostname]
            ssh_only = true
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!("", actual);
    Ok(())
}

#[test]
fn ssh() -> io::Result<()> {
    let hostname = match get_hostname() {
        Some(h) => h,
        None => return hostname_not_tested(),
    };
    let output = common::render_module("hostname")
        .env_clear()
        .use_config(toml::toml! {
            [hostname]
            ssh_only = true
        })
        .env("SSH_CONNECTION", "something")
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!("on {} ", style().paint(hostname));
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn prefix() -> io::Result<()> {
    let hostname = match get_hostname() {
        Some(h) => h,
        None => return hostname_not_tested(),
    };
    let output = common::render_module("hostname")
        .env_clear()
        .use_config(toml::toml! {
            [hostname]
            ssh_only = false
            prefix = "<"
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!("on {} ", style().paint(format!("<{}", hostname)));
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn suffix() -> io::Result<()> {
    let hostname = match get_hostname() {
        Some(h) => h,
        None => return hostname_not_tested(),
    };
    let output = common::render_module("hostname")
        .env_clear()
        .use_config(toml::toml! {
            [hostname]
            ssh_only = false
            suffix = ">"
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!("on {} ", style().paint(format!("{}>", hostname)));
    assert_eq!(actual, expected);
    Ok(())
}

fn get_hostname() -> Option<String> {
    match gethostname::gethostname().into_string() {
        Ok(hostname) => Some(hostname),
        Err(_) => None,
    }
}

fn style() -> Style {
    Color::Green.bold().dimmed()
}

fn hostname_not_tested() -> io::Result<()> {
    println!(
        "hostname was not tested because gethostname failed! This could be caused by your hostname containing invalid \
         UTF."
    );
    Ok(())
}
