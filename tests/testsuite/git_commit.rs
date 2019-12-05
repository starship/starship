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
    // 7 bytes hexed encoded equals 14 chars
    git_output.truncate(14);
    let expected_hash = str::from_utf8(&git_output).unwrap();

    let output = common::render_module("git_commit")
        .use_config(
            toml::from_str(
                "
                    [git_commit]
                        disabled = false
                ",
            )
            .unwrap(),
        )
        .arg("--path")
        .arg(repo_dir)
        .output()?;

    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!(
        "{}",
        Color::Green.bold().paint(format!("({}) ", expected_hash))
    );

    assert_eq!(expected, actual);
    Ok(())
}
