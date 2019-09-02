use ansi_term::Color;
use git2::Repository;
use std::env;
use std::io;
use std::process::Command;

use crate::common::{self, TestCommand};

#[test]
fn test_changed_truncation_symbol() -> io::Result<()> {
    test_truncate_length_with_config(
        "1337_hello_world",
        15,
        "1337_hello_worl",
        "%",
        "truncation_symbol = \"%\"",
    )
}

#[test]
fn test_no_truncation_symbol() -> io::Result<()> {
    test_truncate_length_with_config(
        "1337_hello_world",
        15,
        "1337_hello_worl",
        "",
        "truncation_symbol = \"\"",
    )
}

#[test]
fn test_multi_char_truncation_symbol() -> io::Result<()> {
    test_truncate_length_with_config(
        "1337_hello_world",
        15,
        "1337_hello_worl",
        "a",
        "truncation_symbol = \"apple\"",
    )
}

#[test]
fn test_ascii_boundary_below() -> io::Result<()> {
    test_truncate_length("1337_hello_world", 15, "1337_hello_worl", "…")
}

#[test]
fn test_ascii_boundary_on() -> io::Result<()> {
    test_truncate_length("1337_hello_world", 16, "1337_hello_world", "")
}

#[test]
fn test_ascii_boundary_above() -> io::Result<()> {
    test_truncate_length("1337_hello_world", 17, "1337_hello_world", "")
}

#[test]
fn test_one() -> io::Result<()> {
    test_truncate_length("1337_hello_world", 1, "1", "…")
}

#[test]
fn test_zero() -> io::Result<()> {
    test_truncate_length("1337_hello_world", 0, "1337_hello_world", "")
}

#[test]
fn test_negative() -> io::Result<()> {
    test_truncate_length("1337_hello_world", -1, "1337_hello_world", "")
}

#[test]
fn test_hindi_truncation() -> io::Result<()> {
    test_truncate_length("नमस्ते", 3, "नमस्", "…")
}

#[test]
fn test_hindi_truncation2() -> io::Result<()> {
    test_truncate_length("नमस्त", 3, "नमस्", "…")
}

#[test]
fn test_japanese_truncation() -> io::Result<()> {
    test_truncate_length("がんばってね", 4, "がんばっ", "…")
}

fn test_truncate_length(
    branch_name: &str,
    truncate_length: i64,
    expected_name: &str,
    truncation_symbol: &str,
) -> io::Result<()> {
    return test_truncate_length_with_config(
        branch_name,
        truncate_length,
        expected_name,
        truncation_symbol,
        "",
    );
}

fn test_truncate_length_with_config(
    branch_name: &str,
    truncate_length: i64,
    expected_name: &str,
    truncation_symbol: &str,
    config_options: &str,
) -> io::Result<()> {
    let fixture_repo_dir = create_fixture_repo()?;
    let repo_dir = common::new_tempdir()?.path().join("rocket");

    Repository::clone(fixture_repo_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();
    Command::new("git")
        .args(&["checkout", "-b", branch_name])
        .current_dir(repo_dir.as_path())
        .output()?;

    let output = common::render_module("git_branch")
        .use_config(
            toml::from_str(&format!(
                "
                    [git_branch]
                        truncation_length = {}
                        {}
                ",
                truncate_length, config_options
            ))
            .unwrap(),
        )
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "on {} ",
        Color::Purple
            .bold()
            .paint(format!("\u{e0a0} {}{}", expected_name, truncation_symbol)),
    );
    assert_eq!(expected, actual);
    Ok(())
}

fn create_fixture_repo() -> io::Result<std::path::PathBuf> {
    let fixture_repo_dir = common::new_tempdir()?.path().join("fixture");
    let fixture = env::current_dir()?.join("tests/fixtures/rocket.bundle");

    Command::new("git")
        .args(&["config", "--global", "user.email", "starship@example.com"])
        .output()?;

    Command::new("git")
        .args(&["config", "--global", "user.name", "starship"])
        .output()?;

    Command::new("git")
        .args(&[
            "clone",
            "-b",
            "master",
            &fixture.to_str().unwrap(),
            fixture_repo_dir.to_str().unwrap(),
        ])
        .output()?;

    Ok(fixture_repo_dir)
}
