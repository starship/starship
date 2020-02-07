use ansi_term::Color;
use std::process::Command;
use std::{io, str};

use crate::common::{self, TestCommand};

#[test]
fn test_render_commit_hash() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    let mut git_output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .current_dir(repo_dir.as_path())
        .output()?
        .stdout;
    git_output.truncate(7);
    let expected_hash = str::from_utf8(&git_output).unwrap();

    let output = common::render_module("git_commit")
        .use_config(toml::toml! {
            [git_commit]
                only_detached = false
        })
        .arg("--path")
        .arg(repo_dir)
        .output()?;

    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = Color::Green
        .bold()
        .paint(format!("({}) ", expected_hash))
        .to_string();

    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn test_render_commit_hash_len_override() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    let mut git_output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .current_dir(repo_dir.as_path())
        .output()?
        .stdout;
    git_output.truncate(14);
    let expected_hash = str::from_utf8(&git_output).unwrap();

    let output = common::render_module("git_commit")
        .use_config(toml::toml! {
            [git_commit]
                only_detached = false
                commit_hash_length = 14
        })
        .arg("--path")
        .arg(repo_dir)
        .output()?;

    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = Color::Green
        .bold()
        .paint(format!("({}) ", expected_hash))
        .to_string();

    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn test_render_commit_hash_only_detached_on_branch() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    let output = common::render_module("git_commit")
        .arg("--path")
        .arg(repo_dir)
        .output()?;

    let actual = String::from_utf8(output.stdout).unwrap();

    assert_eq!("", actual);
    Ok(())
}

#[test]
fn test_render_commit_hash_only_detached_on_detached() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    Command::new("git")
        .args(&["checkout", "@~1"])
        .current_dir(repo_dir.as_path())
        .output()?;

    let mut git_output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .current_dir(repo_dir.as_path())
        .output()?
        .stdout;
    git_output.truncate(7);
    let expected_hash = str::from_utf8(&git_output).unwrap();

    let output = common::render_module("git_commit")
        .arg("--path")
        .arg(repo_dir)
        .output()?;

    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = Color::Green
        .bold()
        .paint(format!("({}) ", expected_hash))
        .to_string();

    assert_eq!(expected, actual);
    Ok(())
}
