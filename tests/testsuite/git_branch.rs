use ansi_term::Color;
use remove_dir_all::remove_dir_all;
use std::io;
use std::process::Command;

use crate::common::{self, TestCommand};

#[test]
fn show_nothing_on_empty_dir() -> io::Result<()> {
    let repo_dir = tempfile::tempdir()?;

    let output = common::render_module("git_branch")
        .arg("--path")
        .arg(repo_dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    repo_dir.close()
}

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

#[test]
fn test_format_no_branch() -> io::Result<()> {
    test_format("1337_hello_world", "no_branch", "", "no_branch")
}

#[test]
fn test_format_just_branch_name() -> io::Result<()> {
    test_format("1337_hello_world", "$branch", "", "1337_hello_world")
}

#[test]
fn test_format_just_branch_name_color() -> io::Result<()> {
    test_format(
        "1337_hello_world",
        "[$branch](bold blue)",
        "",
        Color::Blue.bold().paint("1337_hello_world").to_string(),
    )
}

#[test]
fn test_format_mixed_colors() -> io::Result<()> {
    test_format(
        "1337_hello_world",
        "branch: [$branch](bold blue) [THE COLORS](red) ",
        "",
        format!(
            "branch: {} {} ",
            Color::Blue.bold().paint("1337_hello_world").to_string(),
            Color::Red.paint("THE COLORS").to_string()
        ),
    )
}

#[test]
fn test_format_symbol_style() -> io::Result<()> {
    test_format(
        "1337_hello_world",
        "$symbol[$branch]($style)",
        r#"
            symbol = "git: "
            style = "green"
        "#,
        format!(
            "git: {}",
            Color::Green.paint("1337_hello_world").to_string(),
        ),
    )
}

#[test]
fn test_works_with_unborn_master() -> io::Result<()> {
    let repo_dir = tempfile::tempdir()?.into_path();

    Command::new("git")
        .args(&["init"])
        .current_dir(&repo_dir)
        .output()?;

    let output = common::render_module("git_branch")
        .arg("--path")
        .arg(&repo_dir)
        .output()
        .unwrap();
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "on {} ",
        Color::Purple.bold().paint(format!("\u{e0a0} {}", "master")),
    );
    assert_eq!(expected, actual);
    remove_dir_all(repo_dir)
}

fn test_truncate_length(
    branch_name: &str,
    truncate_length: i64,
    expected_name: &str,
    truncation_symbol: &str,
) -> io::Result<()> {
    test_truncate_length_with_config(
        branch_name,
        truncate_length,
        expected_name,
        truncation_symbol,
        "",
    )
}

fn test_truncate_length_with_config(
    branch_name: &str,
    truncate_length: i64,
    expected_name: &str,
    truncation_symbol: &str,
    config_options: &str,
) -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

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
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!(
        "on {} ",
        Color::Purple
            .bold()
            .paint(format!("\u{e0a0} {}{}", expected_name, truncation_symbol)),
    );
    assert_eq!(expected, actual);
    remove_dir_all(repo_dir)
}

fn test_format<T: AsRef<str>>(
    branch_name: &str,
    format: &str,
    config_options: &str,
    expected: T,
) -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    Command::new("git")
        .args(&["checkout", "-b", branch_name])
        .current_dir(repo_dir.as_path())
        .output()?;

    let output = common::render_module("git_branch")
        .use_config(
            toml::from_str(&format!(
                r#"
                    [git_branch]
                        format = "{}"
                        {}
                "#,
                format, config_options
            ))
            .unwrap(),
        )
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    assert_eq!(expected.as_ref(), actual);
    remove_dir_all(repo_dir)
}
