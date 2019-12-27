use std::fs::File;
use std::io::{self, Write};

use ansi_term::Color;
use tempfile;

use crate::common::{self, TestCommand};

#[test]
fn no_region_set() -> io::Result<()> {
    let output = common::render_module("aws")
        .env("PATH", env!("PATH"))
        .output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn region_set() -> io::Result<()> {
    let output = common::render_module("aws")
        .env("AWS_REGION", "ap-northeast-2")
        .output()?;
    let expected = format!("on {} ", Color::Yellow.bold().paint("☁️  ap-northeast-2"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn region_set_with_alias() -> io::Result<()> {
    let output = common::render_module("aws")
        .env("AWS_REGION", "ap-southeast-2")
        .use_config(toml::toml! {
            [aws.region_aliases]
            ap-southeast-2 = "au"
        })
        .output()?;
    let expected = format!("on {} ", Color::Yellow.bold().paint("☁️  au"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn default_region_set() -> io::Result<()> {
    let output = common::render_module("aws")
        .env("AWS_REGION", "ap-northeast-2")
        .env("AWS_DEFAULT_REGION", "ap-northeast-1")
        .output()?;
    let expected = format!("on {} ", Color::Yellow.bold().paint("☁️  ap-northeast-1"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn profile_set() -> io::Result<()> {
    let output = common::render_module("aws")
        .env("AWS_PROFILE", "astronauts")
        .output()?;
    let expected = format!("on {} ", Color::Yellow.bold().paint("☁️  astronauts"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn profile_and_region_set() -> io::Result<()> {
    let output = common::render_module("aws")
        .env("AWS_PROFILE", "astronauts")
        .env("AWS_REGION", "ap-northeast-2")
        .output()?;
    let expected = format!(
        "on {} ",
        Color::Yellow.bold().paint("☁️  astronauts(ap-northeast-2)")
    );
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn default_profile_set() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let config_path = dir.path().join("config");
    let mut file = File::create(&config_path)?;

    file.write_all(
        "[default]
region = us-east-1

[profile astronauts]
region = us-east-2
"
        .as_bytes(),
    )?;

    let output = common::render_module("aws")
        .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
        .output()?;
    let expected = format!("on {} ", Color::Yellow.bold().paint("☁️  us-east-1"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn profile_and_config_set() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let config_path = dir.path().join("config");
    let mut file = File::create(&config_path)?;

    file.write_all(
        "[default]
region = us-east-1

[profile astronauts]
region = us-east-2
"
        .as_bytes(),
    )?;

    let output = common::render_module("aws")
        .env("AWS_CONFIG_FILE", config_path.to_string_lossy().as_ref())
        .env("AWS_PROFILE", "astronauts")
        .use_config(toml::toml! {
            [aws]
        })
        .output()?;
    let expected = format!(
        "on {} ",
        Color::Yellow.bold().paint("☁️  astronauts(us-east-2)")
    );
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn profile_and_region_set_with_display_all() -> io::Result<()> {
    let output = common::render_module("aws")
        .env("AWS_PROFILE", "astronauts")
        .env("AWS_REGION", "ap-northeast-1")
        .use_config(toml::toml! {
            [aws]
            displayed_items = "all"
        })
        .output()?;
    let expected = format!(
        "on {} ",
        Color::Yellow.bold().paint("☁️  astronauts(ap-northeast-1)")
    );
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn profile_set_with_display_all() -> io::Result<()> {
    let output = common::render_module("aws")
        .env("AWS_PROFILE", "astronauts")
        .use_config(toml::toml! {
            [aws]
            displayed_items = "all"
        })
        .output()?;
    let expected = format!("on {} ", Color::Yellow.bold().paint("☁️  astronauts"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn region_set_with_display_all() -> io::Result<()> {
    let output = common::render_module("aws")
        .env("AWS_REGION", "ap-northeast-1")
        .use_config(toml::toml! {
            [aws]
            displayed_items = "all"
        })
        .output()?;
    let expected = format!("on {} ", Color::Yellow.bold().paint("☁️  ap-northeast-1"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn profile_and_region_set_with_display_region() -> io::Result<()> {
    let output = common::render_module("aws")
        .env("AWS_PROFILE", "astronauts")
        .env("AWS_DEFAULT_REGION", "ap-northeast-1")
        .use_config(toml::toml! {
            [aws]
            displayed_items = "region"
        })
        .output()?;
    let expected = format!("on {} ", Color::Yellow.bold().paint("☁️  ap-northeast-1"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn profile_and_region_set_with_display_profile() -> io::Result<()> {
    let output = common::render_module("aws")
        .env("AWS_PROFILE", "astronauts")
        .env("AWS_REGION", "ap-northeast-1")
        .use_config(toml::toml! {
            [aws]
            displayed_items = "profile"
        })
        .output()?;
    let expected = format!("on {} ", Color::Yellow.bold().paint("☁️  astronauts"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn region_set_with_display_profile() -> io::Result<()> {
    let output = common::render_module("aws")
        .env("AWS_REGION", "ap-northeast-1")
        .use_config(toml::toml! {
            [aws]
            displayed_items = "profile"
        })
        .output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn region_not_set_with_display_region() -> io::Result<()> {
    let output = common::render_module("aws")
        .use_config(toml::toml! {
            [aws]
            displayed_items = "region"
        })
        .output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    Ok(())
}
