use ansi_term::Color;
use git2::Repository;
use std::env;
use std::fs::{self, File};
use std::io;
use std::process::Command;

use crate::common;

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

#[test]
#[ignore]
fn shows_behind_count() -> io::Result<()> {
    let fixture_repo_dir = create_fixture_repo()?;
    let repo_dir = common::new_tempdir()?.path().join("rocket");

    Repository::clone(fixture_repo_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    Command::new("git")
        .args(&["reset", "--hard", "HEAD^"])
        .current_dir(repo_dir.as_path())
        .output()?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = Color::Red
        .bold()
        .paint(format!("[{}] ", "⇣1"))
        .to_string();

    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn shows_ahead_count() -> io::Result<()> {
    let fixture_repo_dir = create_fixture_repo()?;
    let repo_dir = common::new_tempdir()?.path().join("rocket");

    Repository::clone(fixture_repo_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    File::create(repo_dir.join("readme.md"))?;

    Command::new("git")
        .args(&["commit", "-am", "Update readme"])
        .current_dir(&repo_dir)
        .output()?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = Color::Red
        .bold()
        .paint(format!("[{}] ", "⇡1"))
        .to_string();

    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn shows_diverged() -> io::Result<()> {
    let fixture_repo_dir = create_fixture_repo()?;
    let repo_dir = common::new_tempdir()?.path().join("rocket");

    Repository::clone(fixture_repo_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    Command::new("git")
        .args(&["reset", "--hard", "HEAD^"])
        .current_dir(repo_dir.as_path())
        .output()?;

    fs::write(repo_dir.join("Cargo.toml"), " ")?;

    Command::new("git")
        .args(&["commit", "-am", "Update readme"])
        .current_dir(repo_dir.as_path())
        .output()?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = Color::Red
        .bold()
        .paint(format!("[{}] ", "⇕⇡1⇣1"))
        .to_string();

    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn shows_conflicted() -> io::Result<()> {
    let fixture_repo_dir = create_fixture_repo()?;
    let repo_dir = common::new_tempdir()?.path().join("rocket");

    Repository::clone(fixture_repo_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

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
#[ignore]
fn shows_untracked_file() -> io::Result<()> {
    let fixture_repo_dir = create_fixture_repo()?;
    let repo_dir = common::new_tempdir()?.path().join("rocket");

    Repository::clone(fixture_repo_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    File::create(repo_dir.join("license"))?;

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
#[ignore]
fn doesnt_show_untracked_file_if_disabled() -> io::Result<()> {
    let fixture_repo_dir = create_fixture_repo()?;
    let repo_dir = common::new_tempdir()?.path().join("rocket");

    Repository::clone(fixture_repo_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    File::create(repo_dir.join("license"))?;

    Command::new("git")
        .args(&["config", "status.showUntrackedFiles", "no"])
        .current_dir(repo_dir.as_path())
        .output()?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = "";

    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn shows_stashed() -> io::Result<()> {
    let fixture_repo_dir = create_fixture_repo()?;
    let repo_dir = common::new_tempdir()?.path().join("rocket");

    Repository::clone(fixture_repo_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    File::create(repo_dir.join("readme.md"))?;

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
#[ignore]
fn shows_modified() -> io::Result<()> {
    let fixture_repo_dir = create_fixture_repo()?;
    let repo_dir = common::new_tempdir()?.path().join("rocket");

    Repository::clone(fixture_repo_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    File::create(repo_dir.join("readme.md"))?;

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
#[ignore]
fn shows_staged_file() -> io::Result<()> {
    let fixture_repo_dir = create_fixture_repo()?;
    let repo_dir = common::new_tempdir()?.path().join("rocket");

    Repository::clone(fixture_repo_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

    File::create(repo_dir.join("license"))?;

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
#[ignore]
fn shows_renamed_file() -> io::Result<()> {
    let fixture_repo_dir = create_fixture_repo()?;
    let repo_dir = common::new_tempdir()?.path().join("rocket");

    Repository::clone(fixture_repo_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

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
#[ignore]
fn shows_deleted_file() -> io::Result<()> {
    let fixture_repo_dir = create_fixture_repo()?;
    let repo_dir = common::new_tempdir()?.path().join("rocket");

    Repository::clone(fixture_repo_dir.to_str().unwrap(), &repo_dir.as_path()).unwrap();

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
