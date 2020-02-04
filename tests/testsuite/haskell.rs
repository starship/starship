use ansi_term::Color;
use dirs::home_dir;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use tempfile::{self, TempDir};

use crate::common;

#[test]
fn folder_without_stack_yaml() -> io::Result<()> {
    let dir = tempfile::tempdir()?;

    let output = common::render_module("haskell")
        .arg("--path")
        .arg(dir.path())
        .output()?;
    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = "";
    assert_eq!(expected, actual);
    Ok(())
}

#[test]
#[ignore]
#[cfg(not(windows))]
fn folder_with_stack_yaml() -> io::Result<()> {
    let dir = tempfile::tempdir()?;
    create_dummy_haskell_project(&dir, Some("nightly-2019-09-21 # Last GHC 8.6.5"))?;

    let output = if cfg!(windows) {
        let mut app_data = home_dir().unwrap();
        app_data.push("AppData");
        app_data.push("Local");
        eprintln!("{}", app_data.to_str().unwrap());
        common::render_module("haskell")
            .env("HOME", home_dir().unwrap())
            .env("LOCALAPPDATA", app_data)
            .env("STACK_ROOT", r"C:\sr")
            .arg("--path")
            .arg(dir.path())
            .output()?
    } else {
        common::render_module("haskell")
            .env("HOME", home_dir().unwrap())
            .arg("--path")
            .arg(dir.path())
            .output()?
    };

    let actual = String::from_utf8(output.stdout).unwrap();

    let expected = format!("via {} ", Color::Red.bold().paint("λ v8.6.5"));
    assert_eq!(expected, actual);
    Ok(())
}

fn create_dummy_haskell_project(folder: &TempDir, contents: Option<&str>) -> io::Result<()> {
    let cabal_path = folder.path().join("test.cabal");
    File::create(cabal_path)?.sync_all()?;

    let stack_yaml_path = folder.path().join("stack.yaml");

    let mut stack_yaml_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&stack_yaml_path)?;
    write!(stack_yaml_file, "resolver: {}", contents.unwrap_or(""))?;
    stack_yaml_file.sync_data()
}
