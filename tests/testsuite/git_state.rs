use super::common;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::{self, Error, ErrorKind, Write};
use std::process::{Command, Stdio};
use tempfile;

#[test]
#[ignore]
fn shows_rebasing() -> io::Result<()> {
    let repo_dir = create_repo_with_conflict()?;
    let path = path_str(&repo_dir)?;

    run_git_cmd(&["rebase", "other-branch"], Some(path), false)?;

    let output = common::render_module("git_state")
        .current_dir(path)
        .output()?;
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("REBASING 1/1"));

    Ok(())
}

#[test]
#[ignore]
fn shows_merging() -> io::Result<()> {
    let repo_dir = create_repo_with_conflict()?;
    let path = path_str(&repo_dir)?;

    run_git_cmd(&["merge", "other-branch"], Some(path), false)?;

    let output = common::render_module("git_state")
        .current_dir(path)
        .output()?;
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("MERGING"));

    Ok(())
}

#[test]
#[ignore]
fn shows_cherry_picking() -> io::Result<()> {
    let repo_dir = create_repo_with_conflict()?;
    let path = path_str(&repo_dir)?;

    run_git_cmd(&["cherry-pick", "other-branch"], Some(path), false)?;

    let output = common::render_module("git_state")
        .current_dir(path)
        .output()?;
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("CHERRY-PICKING"));

    Ok(())
}

#[test]
#[ignore]
fn shows_bisecting() -> io::Result<()> {
    let repo_dir = create_repo_with_conflict()?;
    let path = path_str(&repo_dir)?;

    run_git_cmd(&["bisect", "start"], Some(path), false)?;

    let output = common::render_module("git_state")
        .current_dir(path)
        .output()?;
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("BISECTING"));

    Ok(())
}

#[test]
#[ignore]
fn shows_reverting() -> io::Result<()> {
    let repo_dir = create_repo_with_conflict()?;
    let path = path_str(&repo_dir)?;

    run_git_cmd(&["revert", "--no-commit", "HEAD~1"], Some(path), false)?;

    let output = common::render_module("git_state")
        .current_dir(path)
        .output()?;
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("REVERTING"));

    Ok(())
}

fn run_git_cmd<A, S>(args: A, dir: Option<&str>, expect_ok: bool) -> io::Result<()>
where
    A: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new("git");
    command
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null());

    if let Some(dir) = dir {
        command.current_dir(dir);
    }

    let status = command.status()?;

    if expect_ok && !status.success() {
        Err(Error::from(ErrorKind::Other))
    } else {
        Ok(())
    }
}

fn create_repo_with_conflict() -> io::Result<tempfile::TempDir> {
    let repo_dir = tempfile::tempdir()?;
    let path = path_str(&repo_dir)?;
    let conflicted_file = repo_dir.path().join("the_file");

    let write_file = |text: &str| {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&conflicted_file)?;
        write!(file, "{}", text)
    };

    // Initialise a new git repo
    run_git_cmd(&["init", "--quiet", path], None, true)?;

    // Set local author info
    run_git_cmd(
        &["config", "--local", "user.email", "starship@example.com"],
        Some(path),
        true,
    )?;
    run_git_cmd(
        &["config", "--local", "user.name", "starship"],
        Some(path),
        true,
    )?;

    // Write a file on master and commit it
    write_file("Version A")?;
    run_git_cmd(&["add", "the_file"], Some(path), true)?;
    run_git_cmd(&["commit", "--message", "Commit A"], Some(path), true)?;

    // Switch to another branch, and commit a change to the file
    run_git_cmd(&["checkout", "-b", "other-branch"], Some(path), true)?;
    write_file("Version B")?;
    run_git_cmd(
        &["commit", "--all", "--message", "Commit B"],
        Some(path),
        true,
    )?;

    // Switch back to master, and commit a third change to the file
    run_git_cmd(&["checkout", "master"], Some(path), true)?;
    write_file("Version C")?;
    run_git_cmd(
        &["commit", "--all", "--message", "Commit C"],
        Some(path),
        true,
    )?;

    Ok(repo_dir)
}

fn path_str(repo_dir: &tempfile::TempDir) -> io::Result<&str> {
    repo_dir
        .path()
        .to_str()
        .ok_or_else(|| Error::from(ErrorKind::Other))
}
