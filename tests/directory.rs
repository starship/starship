use ansi_term::Color;
use git2::Repository;
use starship::segment::Segment;
use std::fs;
use std::io;
use std::path::Path;
use tempfile::TempDir;
use dirs::home_dir;

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
#[ignore]
fn directory_in_home() -> io::Result<()> {
    let dir = home_dir().unwrap().join("starship/engine");
    fs::create_dir_all(&dir)?;

    let expected = Segment::new("dir")
        .set_value("~/starship/engine")
        .set_style(Color::Cyan.bold())
        .output();
    let actual = common::render_segment("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_in_home() -> io::Result<()> {
    let dir = home_dir().unwrap().join("starship/engine/schematics");
    fs::create_dir_all(&dir)?;

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
    let dir = Path::new("/opt");

    let expected = Segment::new("dir")
        .set_value("/opt")
        .set_style(Color::Cyan.bold())
        .output();
    let actual = common::render_segment("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn truncated_directory_in_root() -> io::Result<()> {
    let dir = Path::new("/opt/starship/thrusters/rocket");
    fs::create_dir_all(&dir)?;

    let expected = Segment::new("dir")
        .set_value("starship/thrusters/rocket")
        .set_style(Color::Cyan.bold())
        .output();
    let actual = common::render_segment("dir", &dir);
    assert_eq!(expected, actual);

    Ok(())
}

#[test]
#[ignore]
fn git_repo_root() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
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
#[ignore]
fn directory_in_git_repo() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
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
#[ignore]
fn truncated_directory_in_git_repo() -> io::Result<()> {
    let tmp_dir = TempDir::new()?;
    let repo_dir = tmp_dir.path().join("rocket-controls");
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
