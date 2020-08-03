use std::fs::{create_dir, File};
use std::io::{self, Write};

use ansi_term::Color;

use crate::common::{self, TestCommand};

#[test]
fn account_set() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let active_config_path = dir.path().join("active_config");
    let mut active_config_file = File::create(&active_config_path)?;
    active_config_file.write_all(b"default")?;

    create_dir(dir.path().join("configurations"))?;
    let config_default_path = dir.path().join("configurations/config_default");
    let mut config_default_file = File::create(&config_default_path)?;
    config_default_file.write_all(
        b"[core]
account = foo@example.com
",
    )?;

    let output = common::render_module("gcloud")
        .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy().as_ref())
        .output()?;
    let expected = format!("on {} ", Color::Blue.bold().paint("☁️ foo@example.com"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, expected);
    dir.close()
}

#[test]
fn account_and_region_set() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let active_config_path = dir.path().join("active_config");
    let mut active_config_file = File::create(&active_config_path)?;
    active_config_file.write_all(b"default")?;

    create_dir(dir.path().join("configurations"))?;
    let config_default_path = dir.path().join("configurations/config_default");
    let mut config_default_file = File::create(&config_default_path)?;
    config_default_file.write_all(
        b"[core]
account = foo@example.com

[compute]
region = us-central1
",
    )?;

    let output = common::render_module("gcloud")
        .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy().as_ref())
        .output()?;
    let expected = format!(
        "on {} ",
        Color::Blue.bold().paint("☁️ foo@example.com(us-central1)")
    );
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, expected);
    dir.close()
}

#[test]
fn account_and_region_set_with_alias() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let active_config_path = dir.path().join("active_config");
    let mut active_config_file = File::create(&active_config_path)?;
    active_config_file.write_all(b"default")?;

    create_dir(dir.path().join("configurations"))?;
    let config_default_path = dir.path().join("configurations/config_default");
    let mut config_default_file = File::create(&config_default_path)?;
    config_default_file.write_all(
        b"[core]
account = foo@example.com

[compute]
region = us-central1
",
    )?;

    let output = common::render_module("gcloud")
        .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy().as_ref())
        .use_config(toml::toml! {
            [gcloud.region_aliases]
            us-central1 = "uc1"
        })
        .output()?;
    let expected = format!("on {} ", Color::Blue.bold().paint("☁️ foo@example.com(uc1)"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, expected);
    dir.close()
}

#[test]
fn active_set() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let active_config_path = dir.path().join("active_config");
    let mut active_config_file = File::create(&active_config_path)?;
    active_config_file.write_all(b"default1")?;

    let output = common::render_module("gcloud")
        .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy().as_ref())
        .use_config(toml::toml! {
            [gcloud]
            format = "on [$symbol$active]($style) "
        })
        .output()?;
    let expected = format!("on {} ", Color::Blue.bold().paint("☁️ default1"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, expected);
    dir.close()
}

#[test]
fn project_set() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let active_config_path = dir.path().join("active_config");
    let mut active_config_file = File::create(&active_config_path)?;
    active_config_file.write_all(b"default")?;

    create_dir(dir.path().join("configurations"))?;
    let config_default_path = dir.path().join("configurations/config_default");
    let mut config_default_file = File::create(&config_default_path)?;
    config_default_file.write_all(
        b"[core]
project = abc
",
    )?;

    let output = common::render_module("gcloud")
        .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy().as_ref())
        .use_config(toml::toml! {
            [gcloud]
            format = "on [$symbol$project]($style) "
        })
        .output()?;
    let expected = format!("on {} ", Color::Blue.bold().paint("☁️ abc"));
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, expected);
    dir.close()
}

#[test]
fn region_not_set_with_display_region() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    let output = common::render_module("gcloud")
        .env("CLOUDSDK_CONFIG", dir.path().to_string_lossy().as_ref())
        .use_config(toml::toml! {
            [gcloud]
            format = "on [$symbol$region]($style) "
        })
        .output()?;
    let expected = "";
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(expected, actual);
    dir.close()
}
