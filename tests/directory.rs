extern crate starship;

use ansi_term::{Color, Style};
use git2::Repository;
use starship::modules;
use starship::segment::Segment;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use tempdir::TempDir;

mod common;

#[test]
fn directory_in_git_root() -> io::Result<()> {
    let temp_dir = TempDir::new("starship-tests")?;
    let repo_dir = temp_dir.path().join("starship");
    fs::create_dir(&repo_dir)?;

    let repo = match Repository::init(&repo_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to init: {}", e),
    };

    print!("{:?}", repo.path());
    // match Repository::init(&repo_dir) {
    //     Ok(repo) => repo,
    //     Err(e) => panic!("failed to init: {}", e),
    // };

    // std::thread::sleep_ms(1000000);

    let expected = Segment::new("dir")
        .set_value("starship")
        .set_style(Color::Blue.bold())
        .output();
    let actual = common::render_segment("dir", &repo_dir);
    assert_eq!(expected, actual);

    Ok(())
}
