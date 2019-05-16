use ansi_term::Color;
use dirs::home_dir;
use git2::Repository;
use std::fs;
use std::io;
use std::path::Path;
use tempfile::TempDir;

mod common;

#[test]
fn home_directory() -> io::Result<()> {
    let dir = Path::new("~");

    let expected = format!("in {} ", Color::Cyan.bold().paint("~").to_string());
    let actual = common::render_module("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn directory_in_home() -> io::Result<()> {
    let dir = home_dir().unwrap().join("starship/engine");
    fs::create_dir_all(&dir)?;

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("~/starship/engine").to_string()
    );
    let actual = common::render_module("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_in_home() -> io::Result<()> {
    let dir = home_dir().unwrap().join("starship/engine/schematics");
    fs::create_dir_all(&dir)?;

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint("starship/engine/schematics")
            .to_string()
    );
    let actual = common::render_module("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn root_directory() -> io::Result<()> {
    let dir = Path::new("/");

    let expected = format!("in {} ", Color::Cyan.bold().paint("/").to_string());
    let actual = common::render_module("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn directory_in_root() -> io::Result<()> {
    let dir = Path::new("/tmp");

    let expected = format!("in {} ", Color::Cyan.bold().paint("/tmp").to_string());
    let actual = common::render_module("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_in_root() -> io::Result<()> {
    let dir = Path::new("/tmp/starship/thrusters/rocket");
    fs::create_dir_all(&dir)?;

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint("starship/thrusters/rocket")
            .to_string()
    );
    let actual = common::render_module("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn git_repo_root() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    fs::create_dir(&repo_dir)?;

    Repository::init(&repo_dir).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("rocket-controls").to_string()
    );
    let actual = common::render_module("dir", &repo_dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn directory_in_git_repo() -> io::Result<()> {
    let tmp_dir = TempDir::new_in(home_dir().unwrap())?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let dir = repo_dir.join("src");
    fs::create_dir_all(&dir)?;

    Repository::init(&repo_dir).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan.bold().paint("rocket-controls/src").to_string()
    );
    let actual = common::render_module("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_in_git_repo() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
    let dir = repo_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&dir)?;

    Repository::init(&repo_dir).unwrap();

    let expected = format!(
        "in {} ",
        Color::Cyan
            .bold()
            .paint("src/meters/fuel-gauge")
            .to_string()
    );
    let actual = common::render_module("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}
