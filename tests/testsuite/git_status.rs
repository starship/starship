use ansi_term::{ANSIStrings, Color};
use remove_dir_all::remove_dir_all;
use std::fs::{self, File};
use std::io;
use std::path::PathBuf;
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

fn format_output(symbols: &str) -> String {
    format!("{} ", Color::Red.bold().paint(format!("[{}]", symbols)))
}

#[test]
fn show_nothing_on_empty_dir() -> io::Result<()> {
    let repo_dir = tempfile::tempdir()?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(repo_dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    repo_dir.close()
}

#[test]
#[ignore]
fn shows_behind() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    behind(&repo_dir)?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("⇣");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_behind_with_count() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    behind(&repo_dir)?;

    let output = common::render_module("git_status")
        .use_config(toml::toml! {
            [git_status]
            behind = "⇣$count"
        })
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("⇣1");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_ahead() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    File::create(repo_dir.join("readme.md"))?.sync_all()?;
    ahead(&repo_dir)?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("⇡");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_ahead_with_count() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    File::create(repo_dir.join("readme.md"))?.sync_all()?;
    ahead(&repo_dir)?;

    let output = common::render_module("git_status")
        .use_config(toml::toml! {
            [git_status]
            ahead="⇡$count"
        })
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("⇡1");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_diverged() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    diverge(&repo_dir)?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("⇕");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_diverged_with_count() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    diverge(&repo_dir)?;

    let output = common::render_module("git_status")
        .use_config(toml::toml! {
            [git_status]
            diverged=r"⇕⇡$ahead_count⇣$behind_count"
        })
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("⇕⇡1⇣1");
    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_conflicted() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    create_conflict(&repo_dir)?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("=");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_conflicted_with_count() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    create_conflict(&repo_dir)?;

    let output = common::render_module("git_status")
        .use_config(toml::toml! {
            [git_status]
            conflicted = "=$count"
        })
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("=1");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_untracked_file() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    create_untracked(&repo_dir)?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("?");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_untracked_file_with_count() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    create_untracked(&repo_dir)?;

    let output = common::render_module("git_status")
        .use_config(toml::toml! {
            [git_status]
            untracked = "?$count"
        })
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("?1");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn doesnt_show_untracked_file_if_disabled() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    create_untracked(&repo_dir)?;

    Command::new("git")
        .args(&["config", "status.showUntrackedFiles", "no"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = "";

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_stashed() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;
    barrier();

    create_stash(&repo_dir)?;

    Command::new("git")
        .args(&["reset", "--hard", "HEAD"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("$");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
fn shows_stashed_with_count() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;
    barrier();

    create_stash(&repo_dir)?;
    barrier();

    Command::new("git")
        .args(&["reset", "--hard", "HEAD"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    let output = common::render_module("git_status")
        .use_config(toml::toml! {
            [git_status]
            stashed = r"\$$count"
        })
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("$1");

    assert_eq!(expected, actual);
    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_modified() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    create_modified(&repo_dir)?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("!");

    assert_eq!(expected, actual);
    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_modified_with_count() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    create_modified(&repo_dir)?;

    let output = common::render_module("git_status")
        .use_config(toml::toml! {
            [git_status]
            modified = "!$count"
        })
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("!1");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_staged_file() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    create_staged(&repo_dir)?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("+");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_staged_file_with_count() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    create_staged(&repo_dir)?;

    let output = common::render_module("git_status")
        .use_config(toml::toml! {
            [git_status]
            staged = "+[$count](green)"
        })
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!(
        "{} ",
        ANSIStrings(&[
            Color::Red.bold().paint("[+"),
            Color::Green.paint("1"),
            Color::Red.bold().paint("]"),
        ])
    );

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_renamed_file() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    create_renamed(&repo_dir)?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("»");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_renamed_file_with_count() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    create_renamed(&repo_dir)?;

    let output = common::render_module("git_status")
        .use_config(toml::toml! {
            [git_status]
            renamed = "»$count"
        })
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("»1");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_deleted_file() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    create_deleted(&repo_dir)?;

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("✘");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn shows_deleted_file_with_count() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;

    create_deleted(&repo_dir)?;

    let output = common::render_module("git_status")
        .use_config(toml::toml! {
            [git_status]
            deleted = "✘$count"
        })
        .arg("--path")
        .arg(&repo_dir)
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format_output("✘1");

    assert_eq!(expected, actual);

    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn prefix() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;
    File::create(repo_dir.join("prefix"))?.sync_all()?;
    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
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
    remove_dir_all(repo_dir)
}

#[test]
#[ignore]
fn suffix() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;
    File::create(repo_dir.join("suffix"))?.sync_all()?;
    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
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
    remove_dir_all(repo_dir)
}

// Whenever a file is manually renamed, git itself ('git status') does not treat such file as renamed,
// but as untracked instead. The following test checks if manually deleted and manually renamed
// files are tracked by git_status module in the same way 'git status' does.
#[test]
#[ignore]
fn ignore_manually_renamed() -> io::Result<()> {
    let repo_dir = common::create_fixture_repo()?;
    File::create(repo_dir.join("a"))?.sync_all()?;
    File::create(repo_dir.join("b"))?.sync_all()?;
    Command::new("git")
        .args(&["add", "--all"])
        .current_dir(&repo_dir)
        .output()?;
    Command::new("git")
        .args(&["commit", "-m", "add new files"])
        .current_dir(&repo_dir)
        .output()?;

    fs::remove_file(repo_dir.join("a"))?;
    fs::rename(repo_dir.join("b"), repo_dir.join("c"))?;
    barrier();

    let output = common::render_module("git_status")
        .arg("--path")
        .arg(&repo_dir)
        .env_clear()
        .use_config(toml::toml! {
            [git_status]
            prefix = ""
            suffix = ""
            style = ""
            ahead = "A"
            deleted = "D"
            untracked = "U"
        })
        .output()?;

    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = "ADU";
    assert_eq!(actual, expected);

    remove_dir_all(repo_dir)
}

fn ahead(repo_dir: &PathBuf) -> io::Result<()> {
    File::create(repo_dir.join("readme.md"))?.sync_all()?;

    Command::new("git")
        .args(&["commit", "-am", "Update readme"])
        .current_dir(&repo_dir)
        .output()?;
    barrier();

    Ok(())
}

fn behind(repo_dir: &PathBuf) -> io::Result<()> {
    Command::new("git")
        .args(&["reset", "--hard", "HEAD^"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    Ok(())
}

fn diverge(repo_dir: &PathBuf) -> io::Result<()> {
    Command::new("git")
        .args(&["reset", "--hard", "HEAD^"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    fs::write(repo_dir.join("Cargo.toml"), " ")?;

    Command::new("git")
        .args(&["commit", "-am", "Update readme"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    Ok(())
}

fn create_conflict(repo_dir: &PathBuf) -> io::Result<()> {
    Command::new("git")
        .args(&["reset", "--hard", "HEAD^"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    fs::write(repo_dir.join("readme.md"), "# goodbye")?;

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

    Ok(())
}

fn create_stash(repo_dir: &PathBuf) -> io::Result<()> {
    File::create(repo_dir.join("readme.md"))?.sync_all()?;
    barrier();

    Command::new("git")
        .args(&["stash", "--all"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    Ok(())
}

fn create_untracked(repo_dir: &PathBuf) -> io::Result<()> {
    File::create(repo_dir.join("license"))?.sync_all()?;

    Ok(())
}

fn create_modified(repo_dir: &PathBuf) -> io::Result<()> {
    File::create(repo_dir.join("readme.md"))?.sync_all()?;

    Ok(())
}

fn create_staged(repo_dir: &PathBuf) -> io::Result<()> {
    File::create(repo_dir.join("license"))?.sync_all()?;

    Command::new("git")
        .args(&["add", "."])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    Ok(())
}

fn create_renamed(repo_dir: &PathBuf) -> io::Result<()> {
    Command::new("git")
        .args(&["mv", "readme.md", "readme.md.bak"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    Command::new("git")
        .args(&["add", "-A"])
        .current_dir(repo_dir.as_path())
        .output()?;
    barrier();

    Ok(())
}

fn create_deleted(repo_dir: &PathBuf) -> io::Result<()> {
    fs::remove_file(repo_dir.join("readme.md"))?;

    Ok(())
}
