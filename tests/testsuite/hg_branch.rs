use ansi_term::{Color, Style};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, io};
use tempfile;

use crate::common::{self, TestCommand};

enum Expect<'a> {
    BranchName(&'a str),
    Empty,
    NoTruncation,
    Symbol(&'a str),
    Style(Style),
    TruncationSymbol(&'a str),
}

#[test]
#[ignore]
fn test_hg_get_branch_fails() -> io::Result<()> {
    let tempdir = tempfile::tempdir()?;

    // Create a fake corrupted mercurial repo.
    let hgdir = tempdir.path().join(".hg");
    fs::create_dir(&hgdir)?;
    fs::write(&hgdir.join("requires"), "fake-corrupted-repo")?;

    expect_hg_branch_with_config(
        tempdir.path(),
        "",
        &[Expect::BranchName(&"(no branch)"), Expect::NoTruncation],
    )
}

#[test]
#[ignore]
fn test_hg_get_branch_autodisabled() -> io::Result<()> {
    let tempdir = tempfile::tempdir()?;
    expect_hg_branch_with_config(tempdir.path(), "", &[Expect::Empty])
}

#[test]
#[ignore]
fn test_hg_bookmark() -> io::Result<()> {
    let tempdir = tempfile::tempdir()?;
    let repo_dir = create_fixture_hgrepo(&tempdir)?;
    run_hg(&["bookmark", "bookmark-101"], &repo_dir)?;
    expect_hg_branch_with_config(
        &repo_dir,
        "",
        &[Expect::BranchName(&"bookmark-101"), Expect::NoTruncation],
    )
}

#[test]
#[ignore]
fn test_default_truncation_symbol() -> io::Result<()> {
    let tempdir = tempfile::tempdir()?;
    let repo_dir = create_fixture_hgrepo(&tempdir)?;
    run_hg(&["branch", "-f", "branch-name-101"], &repo_dir)?;
    run_hg(
        &[
            "commit",
            "-m",
            "empty commit 101",
            "-u",
            "fake user <fake@user>",
        ],
        &repo_dir,
    )?;
    expect_hg_branch_with_config(
        &repo_dir,
        "truncation_length = 14",
        &[Expect::BranchName(&"branch-name-10")],
    )
}

#[test]
#[ignore]
fn test_configured_symbols() -> io::Result<()> {
    let tempdir = tempfile::tempdir()?;
    let repo_dir = create_fixture_hgrepo(&tempdir)?;
    run_hg(&["branch", "-f", "branch-name-121"], &repo_dir)?;
    run_hg(
        &[
            "commit",
            "-m",
            "empty commit 121",
            "-u",
            "fake user <fake@user>",
        ],
        &repo_dir,
    )?;
    expect_hg_branch_with_config(
        &repo_dir,
        r#"
          truncation_length = 14
          truncation_symbol = "%"
        "#,
        &[
            Expect::BranchName(&"branch-name-12"),
            Expect::TruncationSymbol(&"%"),
        ],
    )
}

#[test]
#[ignore]
fn test_configured_format() -> io::Result<()> {
    let tempdir = tempfile::tempdir()?;
    let repo_dir = create_fixture_hgrepo(&tempdir)?;
    run_hg(&["branch", "-f", "branch-name-131"], &repo_dir)?;
    run_hg(
        &[
            "commit",
            "-m",
            "empty commit 131",
            "-u",
            "fake user <fake@user>",
        ],
        &repo_dir,
    )?;

    expect_hg_branch_with_config(
        &repo_dir,
        r#"
          format = "on ${styled?value=B &style=purple bold}${name?style=underline blue} "
        "#,
        &[
            Expect::BranchName(&"branch-name-131"),
            Expect::Style(Color::Blue.underline()),
            Expect::Symbol(&"B"),
            Expect::TruncationSymbol(&""),
        ],
    )
}

fn expect_hg_branch_with_config(
    repo_dir: &Path,
    config_options: &str,
    expectations: &[Expect],
) -> io::Result<()> {
    let output = common::render_module("hg_branch")
        .use_config(toml::from_str(&format!(
            r#"
            [hg_branch]
            {}
            "#,
            config_options
        ))?)
        .arg("--path")
        .arg(repo_dir.to_str().unwrap())
        .output()?;

    let actual = String::from_utf8(output.stdout).unwrap();

    let mut expect_branch_name = "(no branch)";
    let mut expect_style = Color::Purple.bold();
    let mut expect_symbol = "\u{e0a0}";
    let mut expect_truncation_symbol = "…";

    for expect in expectations {
        match expect {
            Expect::Empty => {
                assert_eq!("", actual);
                return Ok(());
            }
            Expect::Symbol(symbol) => {
                expect_symbol = symbol;
            }
            Expect::TruncationSymbol(truncation_symbol) => {
                expect_truncation_symbol = truncation_symbol;
            }
            Expect::NoTruncation => {
                expect_truncation_symbol = "";
            }
            Expect::BranchName(branch_name) => {
                expect_branch_name = branch_name;
            }
            Expect::Style(style) => expect_style = *style,
        }
    }

    let expected = format!(
        "on {} ",
        expect_style.paint(format!(
            "{} {}{}",
            expect_symbol, expect_branch_name, expect_truncation_symbol
        )),
    );
    assert_eq!(expected, actual);
    Ok(())
}

pub fn create_fixture_hgrepo(tempdir: &tempfile::TempDir) -> io::Result<PathBuf> {
    let repo_path = tempdir.path().join("hg-repo");
    let fixture_path = env::current_dir()?.join("tests/fixtures/hg-repo.bundle");

    run_hg(
        &[
            "clone",
            fixture_path.to_str().unwrap(),
            repo_path.to_str().unwrap(),
        ],
        &tempdir.path(),
    )?;

    Ok(repo_path)
}

fn run_hg(args: &[&str], repo_dir: &Path) -> io::Result<()> {
    Command::new("hg")
        .args(args)
        .current_dir(&repo_dir)
        .output()?;
    Ok(())
}
