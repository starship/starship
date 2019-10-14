use ansi_term::Color;
use std::fs::{self, File};
use std::io;
use std::process::Command;

use crate::common::{self, TestCommand};

/// Right after the calls to git the filesystem state may not have finished
/// updating yet causing some of the tests to fail. These barriers are placed
/// after each call to git.
/// This barrier is windows-specific though other operating systems may need it
/// in the future.
#[cfg(not(windows))]
fn barrier() {}
#[cfg(windows)]
fn barrier() {
    std::thread::sleep(std::time::Duration::from_millis(500));
}

#[test]
#[ignore]
fn shows_behind() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    Command::new("git")
        .args(&["reset", "--hard", "HEAD^"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = Color::Red.bold().paint(format!("[{}] ", "⇣")).to_string();

    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn shows_behind_with_count() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    Command::new("git")
        .args(&["reset", "--hard", "HEAD^"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    let output = common::render_module("git_status")
        .use_config(toml::toml! {
            [git_status]
            show_sync_count = true
        })
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = Color::Red.bold().paint(format!("[{}] ", "⇣1")).to_string();

    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn shows_ahead() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    File::create(repo_dir.join("readme.md"))?.sync_all()?;

    Command::new("git")
        .args(&["commit", "-am", "Update readme"])
        .current_dir(&repo_dir)
        .output()?;
    barrier();

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = Color::Red.bold().paint(format!("[{}] ", "⇡")).to_string();

    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn shows_ahead_with_count() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    File::create(repo_dir.join("readme.md"))?.sync_all()?;

    Command::new("git")
        .args(&["commit", "-am", "Update readme"])
        .current_dir(&repo_dir)
        .output()?;
    barrier();

    let output = common::render_module("git_status")
        .use_config(toml::toml! {
            [git_status]
            show_sync_count = true
        })
        .arg("--path")
        .arg(repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = Color::Red.bold().paint(format!("[{}] ", "⇡1")).to_string();

    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn shows_diverged() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    Command::new("git")
        .args(&["reset", "--hard", "HEAD^"])
        .current_dir(repo_dir.as_path())
        .output()?;

    fs::write(repo_dir.join("Cargo.toml"), " ")?;

    Command::new("git")
        .args(&["commit", "-am", "Update readme"])
        .current_dir(repo_dir.as_path())
        .output()?;

    barrier();

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
#[ignore]
fn shows_diverged_with_count() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    Command::new("git")
        .args(&["reset", "--hard", "HEAD^"])
        .current_dir(repo_dir.as_path())
        .output()?;

    fs::write(repo_dir.join("Cargo.toml"), " ")?;

    Command::new("git")
        .args(&["commit", "-am", "Update readme"])
        .current_dir(repo_dir.as_path())
        .output()?;

    barrier();

    let output = common::render_module("git_status")
        .use_config(toml::toml! {
            [git_status]
            show_sync_count = true
        })
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
    let repo_dir = common::create_fixture_repo()?;

    Command::new("git")
        .args(&["reset", "--hard", "HEAD^"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    fs::write(repo_dir.join("readme.md"), "# goodbye")?;
    barrier();

    Command::new("git")
        .args(&["add", "."])
        .current_dir(repo_dir.as_path())
        .output()?;

    barrier();

    Command::new("git")
        .args(&["commit", "-m", "Change readme"])
        .current_dir(repo_dir.as_path())
        .output()?;

    barrier();
    Command::new("git")
        .args(&["pull", "--rebase"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

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
    let repo_dir = common::create_fixture_repo()?;

    File::create(repo_dir.join("license"))?.sync_all()?;

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
    let repo_dir = common::create_fixture_repo()?;

    File::create(repo_dir.join("license"))?.sync_all()?;

    Command::new("git")
        .args(&["config", "status.showUntrackedFiles", "no"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

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
    let repo_dir = common::create_fixture_repo()?;

    File::create(repo_dir.join("readme.md"))?.sync_all()?;

    Command::new("git")
        .arg("stash")
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

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
    let repo_dir = common::create_fixture_repo()?;

    File::create(repo_dir.join("readme.md"))?.sync_all()?;

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
    let repo_dir = common::create_fixture_repo()?;

    File::create(repo_dir.join("license"))?.sync_all()?;

    Command::new("git")
        .args(&["add", "."])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

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
    let repo_dir = common::create_fixture_repo()?;

    Command::new("git")
        .args(&["mv", "readme.md", "readme.md.bak"])
        .current_dir(repo_dir.as_path())
        .output()?;

    Command::new("git")
        .args(&["add", "-A"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

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
    let repo_dir = common::create_fixture_repo()?;

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

#[test]
#[ignore]
fn prefix() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;
    File::create(repo_dir.join("prefix"))?.sync_all()?;
    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .env_clear()
        .use_config(toml::toml! {
            [git_status]
            prefix = "("
            style = ""
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = "(";
    assert!(actual.starts_with(&expected));
    Ok(())
}

#[test]
#[ignore]
fn suffix() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;
    File::create(repo_dir.join("suffix"))?.sync_all()?;
    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir)
        .env_clear()
        .use_config(toml::toml! {
            [git_status]
            suffix = ")"
            style = ""
        })
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = ")";
    assert!(actual.ends_with(&expected));
    Ok(())
}
