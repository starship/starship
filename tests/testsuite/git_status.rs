use ansi_term::Color;
use git2::Repository;
use std::env;
use std::fs;
use std::io;
use std::process::Command;
use tempfile::TempDir;

use crate::common;

#[test]
fn shows_behind_count() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let fixture_dir = env::current_dir()?.join("tests/fixtures/rocket");

    fs::create_dir(&repo_dir)?;

    Repository::clone(fixture_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    Command::new("git")
        .args(&["reset", "--hard", "HEAD^"])
        .current_dir(repo_dir.as_path())
        .output()?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = Color::Red.bold().paint(format!("[{}] ", "⇣1")).to_string();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn shows_ahead_count() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let fixture_dir = env::current_dir()?.join("tests/fixtures/rocket");

    fs::create_dir(&repo_dir)?;

    Repository::clone(fixture_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    fs::write(repo_dir.join("readme.md"), "")?;

    Command::new("git")
        .args(&["commit", "-am", "Add license"])
        .current_dir(repo_dir.as_path())
        .output()?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = Color::Red.bold().paint(format!("[{}] ", "⇡1")).to_string();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn shows_diverged() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let fixture_dir = env::current_dir()?.join("tests/fixtures/rocket");

    fs::create_dir(&repo_dir)?;

    Repository::clone(fixture_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    Command::new("git")
        .args(&["reset", "--hard", "HEAD^"])
        .current_dir(repo_dir.as_path())
        .output()?;

    fs::write(repo_dir.join("Cargo.toml"), " ")?;

    Command::new("git")
        .args(&["commit", "-am", "Update Cargo.toml"])
        .current_dir(repo_dir.as_path())
        .output()?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = Color::Red.bold().paint(format!("[{}] ", "⇕")).to_string();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn shows_conflicted() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let fixture_dir = env::current_dir()?.join("tests/fixtures/rocket");

    fs::create_dir(&repo_dir)?;

    Repository::clone(fixture_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    Command::new("git")
        .args(&["reset", "--hard", "HEAD^"])
        .current_dir(repo_dir.as_path())
        .output()?;

    fs::write(repo_dir.join("readme.md"), "# goodbye")?;

    Command::new("git")
        .args(&["add", "."])
        .current_dir(repo_dir.as_path())
        .output()?;

    Command::new("git")
        .args(&["commit", "-m", "Change readme"])
        .current_dir(repo_dir.as_path())
        .output()?;

    Command::new("git")
        .args(&["pull", "--rebase"])
        .current_dir(repo_dir.as_path())
        .output()?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = Color::Red.bold().paint(format!("[{}] ", "=")).to_string();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn shows_untracked_file() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let fixture_dir = env::current_dir()?.join("tests/fixtures/rocket");

    fs::create_dir(&repo_dir)?;

    Repository::clone(fixture_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    fs::write(repo_dir.join("license"), "")?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = Color::Red.bold().paint(format!("[{}] ", "?")).to_string();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn shows_stashed() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let fixture_dir = env::current_dir()?.join("tests/fixtures/rocket");

    fs::create_dir(&repo_dir)?;

    Repository::clone(fixture_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    fs::write(repo_dir.join("readme.md"), "")?;

    Command::new("git")
        .arg("stash")
        .current_dir(repo_dir.as_path())
        .output()?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = Color::Red.bold().paint(format!("[{}] ", "$")).to_string();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn shows_modified() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let fixture_dir = env::current_dir()?.join("tests/fixtures/rocket");

    fs::create_dir(&repo_dir)?;

    Repository::clone(fixture_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    fs::write(repo_dir.join("readme.md"), "# goodbye")?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = Color::Red.bold().paint(format!("[{}] ", "!")).to_string();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn shows_added_file() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let fixture_dir = env::current_dir()?.join("tests/fixtures/rocket");

    fs::create_dir(&repo_dir)?;

    Repository::clone(fixture_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    fs::write(repo_dir.join("license"), "")?;

    Command::new("git")
        .args(&["add", "."])
        .current_dir(repo_dir.as_path())
        .output()?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = Color::Red.bold().paint(format!("[{}] ", "+")).to_string();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn shows_renamed_file() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let fixture_dir = env::current_dir()?.join("tests/fixtures/rocket");

    fs::create_dir(&repo_dir)?;

    Repository::clone(fixture_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    Command::new("git")
        .args(&["mv", "readme.md", "readme.md.bak"])
        .current_dir(repo_dir.as_path())
        .output()?;

    Command::new("git")
        .args(&["add", "-A"])
        .current_dir(repo_dir.as_path())
        .output()?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = Color::Red.bold().paint(format!("[{}] ", "»")).to_string();
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
fn shows_deleted_file() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let fixture_dir = env::current_dir()?.join("tests/fixtures/rocket");

    fs::create_dir(&repo_dir)?;

    Repository::clone(fixture_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    fs::remove_file(repo_dir.join("readme.md"))?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = Color::Red.bold().paint(format!("[{}] ", "✘")).to_string();
    assert_eq!(expected, actual);
    Ok(())
}
