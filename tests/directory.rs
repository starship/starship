use ansi_term::Color;
use git2::Repository;
use starship::segment::Segment;
use std::fs;
use std::io;
use std::path::Path;
use tempdir::TempDir;

mod common;

#[test]
fn home_directory() -> io::Result<()> {
    let dir = Path::new("~");

    let expected = Segment::new("dir")
        .set_value("~")
        .set_style(Color::Cyan.bold())
        .output();
    let actual = common::render_segment("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn directory_in_home() -> io::Result<()> {
    let dir = Path::new("~/starship/engine");

    let expected = Segment::new("dir")
        .set_value("~/starship/engine")
        .set_style(Color::Cyan.bold())
        .output();
    let actual = common::render_segment("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn truncated_directory_in_home() -> io::Result<()> {
    let dir = Path::new("~/starship/engine/schematics");

    let expected = Segment::new("dir")
        .set_value("starship/engine/schematics")
        .set_style(Color::Cyan.bold())
        .output();
    let actual = common::render_segment("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn root_directory() -> io::Result<()> {
    let dir = Path::new("/");

    let expected = Segment::new("dir")
        .set_value("/")
        .set_style(Color::Cyan.bold())
        .output();
    let actual = common::render_segment("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn directory_in_root() -> io::Result<()> {
    let dir = Path::new("/private");

    let expected = Segment::new("dir")
        .set_value("/private")
        .set_style(Color::Cyan.bold())
        .output();
    let actual = common::render_segment("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn truncated_directory_in_root() -> io::Result<()> {
    let dir = Path::new("/private/var/folders/3s");

    let expected = Segment::new("dir")
        .set_value("var/folders/3s")
        .set_style(Color::Cyan.bold())
        .output();
    let actual = common::render_segment("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn git_repo_root() -> io::Result<()> {
    let temp_dir = TempDir::new("starship")?;
    let repo_dir = temp_dir.path().join("rocket-controls");
    fs::create_dir(&repo_dir)?;

    Repository::init(&repo_dir).unwrap();

    let expected = Segment::new("dir")
        .set_value("rocket-controls")
        .set_style(Color::Cyan.bold())
        .output();
    let actual = common::render_segment("dir", &repo_dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn directory_in_git_repo() -> io::Result<()> {
    let temp_dir = TempDir::new("starship")?;
    let repo_dir = temp_dir.path().join("rocket-controls");
    let dir = repo_dir.join("src");
    fs::create_dir_all(&dir)?;

    Repository::init(&repo_dir).unwrap();

    let expected = Segment::new("dir")
        .set_value("rocket-controls/src")
        .set_style(Color::Cyan.bold())
        .output();
    let actual = common::render_segment("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
fn truncated_directory_in_git_repo() -> io::Result<()> {
    let temp_dir = TempDir::new("starship")?;
    let repo_dir = temp_dir.path().join("rocket-controls");
    let dir = repo_dir.join("src/meters/fuel-gauge");
    fs::create_dir_all(&dir)?;

    Repository::init(&repo_dir).unwrap();

    let expected = Segment::new("dir")
        .set_value("src/meters/fuel-gauge")
        .set_style(Color::Cyan.bold())
        .output();
    let actual = common::render_segment("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}
