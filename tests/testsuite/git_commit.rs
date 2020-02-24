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
fn test_render_commit_hash_with_tag_enabled() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    let mut git_commit = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .current_dir(repo_dir.as_path())
        .output()?
        .stdout;
    git_commit.truncate(7);
    let commit_output = str::from_utf8(&git_commit).unwrap().trim();

    let git_tag = Command::new("git")
        .args(&["describe", "--tags", "--exact-match", "HEAD"])
        .current_dir(repo_dir.as_path())
        .output()?
        .stdout;
    let tag_output = str::from_utf8(&git_tag).unwrap().trim();

    let expected_output = format!("{} {}", commit_output, tag_output);

    let output = common::render_module("git_commit")
        .use_config(toml::toml! {
            [git_commit]
                only_detached = false
                tag_disabled = false
                tag_symbol = ""
        })
        .arg("--path")
        .arg(repo_dir)
        .output()?;

    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = Color::Green
        .bold()
        .paint(format!("({}) ", expected_output.trim()))
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

#[test]
fn test_render_commit_hash_only_detached_on_detached_with_tag_enabled() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    Command::new("git")
        .args(&["checkout", "@~1"])
        .current_dir(repo_dir.as_path())
        .output()?;

    Command::new("git")
        .args(&["tag", "tagOnDetached", "-m", "Testing tags on detached"])
        .current_dir(repo_dir.as_path())
        .output()?;

    let mut git_commit = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .current_dir(repo_dir.as_path())
        .output()?
        .stdout;
    git_commit.truncate(7);
    let commit_output = str::from_utf8(&git_commit).unwrap().trim();

    let git_tag = Command::new("git")
        .args(&["describe", "--tags", "--exact-match", "HEAD"])
        .current_dir(repo_dir.as_path())
        .output()?
        .stdout;
    let tag_output = str::from_utf8(&git_tag).unwrap().trim();

    let expected_output = format!("{} {}", commit_output, tag_output);

    let output = common::render_module("git_commit")
        .use_config(toml::toml! {
            [git_commit]
                tag_disabled = false
                tag_symbol = ""
        })
        .arg("--path")
        .arg(repo_dir)
        .output()?;

    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = Color::Green
        .bold()
        .paint(format!("({}) ", expected_output.trim()))
        .to_string();

    assert_eq!(expected, actual);
    Ok(())
}
