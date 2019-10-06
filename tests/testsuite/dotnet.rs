use super::common;
use std::fs::{DirBuilder, OpenOptions};
use std::io::{self, Error, ErrorKind, Write};
use std::process::{Command, Stdio};
use tempfile::TempDir;

#[test]
#[ignore]
fn shows_nothing_in_directory_with_zero_relevant_files() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    expect_output(&workspace, ".", None)
}

#[test]
#[ignore]
fn shows_latest_in_directory_with_solution() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    touch_path(&workspace, "solution.sln", None)?;
    expect_output(&workspace, ".", Some("•NET v2.2.402"))
}

#[test]
#[ignore]
fn shows_latest_in_directory_with_csproj() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    touch_path(&workspace, "project.csproj", None)?;
    expect_output(&workspace, ".", Some("•NET v2.2.402"))
}

#[test]
#[ignore]
fn shows_latest_in_directory_with_fsproj() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    touch_path(&workspace, "project.fsproj", None)?;
    expect_output(&workspace, ".", Some("•NET v2.2.402"))
}

#[test]
#[ignore]
fn shows_latest_in_directory_with_xproj() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    touch_path(&workspace, "project.xproj", None)?;
    expect_output(&workspace, ".", Some("•NET v2.2.402"))
}

#[test]
#[ignore]
fn shows_latest_in_directory_with_project_json() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    touch_path(&workspace, "project.json", None)?;
    expect_output(&workspace, ".", Some("•NET v2.2.402"))
}

#[test]
#[ignore]
fn shows_pinned_in_directory_with_global_json() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    let global_json = make_pinned_sdk_json("1.2.3");
    touch_path(&workspace, "global.json", Some(&global_json))?;
    expect_output(&workspace, ".", Some("•NET v1.2.3"))
}

#[test]
#[ignore]
fn shows_pinned_in_project_below_root_with_global_json() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    let global_json = make_pinned_sdk_json("1.2.3");
    touch_path(&workspace, "global.json", Some(&global_json))?;
    touch_path(&workspace, "project/project.csproj", None)?;
    expect_output(&workspace, "project", Some("•NET v1.2.3"))
}

#[test]
#[ignore]
fn shows_pinned_in_deeply_nested_project_within_repository() -> io::Result<()> {
    let workspace = create_workspace(true)?;
    let global_json = make_pinned_sdk_json("1.2.3");
    touch_path(&workspace, "global.json", Some(&global_json))?;
    touch_path(&workspace, "deep/path/to/project/project.csproj", None)?;
    expect_output(&workspace, "deep/path/to/project", Some("•NET v1.2.3"))
}

fn create_workspace(is_repo: bool) -> io::Result<TempDir> {
    let repo_dir = common::new_tempdir()?;

    if is_repo {
        let mut command = Command::new("git");
        command
            .args(&["init", "--quiet"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .current_dir(repo_dir.path());

        if !command.status()?.success() {
            return Err(Error::from(ErrorKind::Other));
        }
    }

    Ok(repo_dir)
}

fn touch_path(workspace: &TempDir, relative_path: &str, contents: Option<&str>) -> io::Result<()> {
    let path = workspace.path().join(relative_path);

    DirBuilder::new().recursive(true).create(
        path.parent()
            .expect("Expected relative_path to be a file in a directory"),
    )?;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)?;
    write!(file, "{}", contents.unwrap_or(""))
    file.sync_data();
}

fn make_pinned_sdk_json(version: &str) -> String {
    let json_text = r#"
        {
            "sdk": {
                "version": "INSERT_VERSION"
            }
        }
    "#;
    json_text.replace("INSERT_VERSION", version)
}

fn expect_output(workspace: &TempDir, run_from: &str, contains: Option<&str>) -> io::Result<()> {
    let run_path = workspace.path().join(run_from);
    let output = common::render_module("dotnet")
        .current_dir(run_path)
        .output()?;
    let text = String::from_utf8(output.stdout).unwrap();

    // This can be helpful for debugging
    eprintln!("The dotnet module showed: {}", text);

    match contains {
        Some(contains) => assert!(text.contains(contains)),
        None => assert!(text.is_empty()),
    }

    Ok(())
}
