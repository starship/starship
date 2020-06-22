use super::common;
use regex::Regex;
use std::fs::{DirBuilder, OpenOptions};
use std::io::{self, Error, ErrorKind, Write};
use std::process::{Command, Stdio};
use tempfile::{self, TempDir};

const DOTNET_OUTPUT_PATTERN: &str = "•NET v\\d+?\\.\\d+?\\.\\d?";
const DOTNET_PINNED_VERSION: &str = "1.2.3";
const DOTNET_PINNED_VERSION_OUTPUT_PATTERN: &str = "•NET v1\\.2\\.3";
const DOTNET_TFM_PATTERN: &str = r"🎯 .+";
const DOTNET_TFM_PINNED_VERSION: &str = r"netstandard2.0";

#[test]
#[ignore]
fn shows_nothing_in_directory_with_zero_relevant_files() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    expect_output(&workspace, ".", None)?;
    workspace.close()
}

#[test]
#[ignore]
fn shows_latest_in_directory_with_directory_build_props_file() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    touch_path(&workspace, "Directory.Build.props", None)?;
    expect_output(&workspace, ".", Some(DOTNET_OUTPUT_PATTERN))?;
    workspace.close()
}

#[test]
#[ignore]
fn shows_latest_in_directory_with_directory_build_targets_file() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    touch_path(&workspace, "Directory.Build.targets", None)?;
    expect_output(&workspace, ".", Some(DOTNET_OUTPUT_PATTERN))?;
    workspace.close()
}

#[test]
#[ignore]
fn shows_latest_in_directory_with_packages_props_file() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    touch_path(&workspace, "Packages.props", None)?;
    expect_output(&workspace, ".", Some(DOTNET_OUTPUT_PATTERN))?;
    workspace.close()
}

#[test]
#[ignore]
fn shows_latest_in_directory_with_solution() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    touch_path(&workspace, "solution.sln", None)?;
    expect_output(&workspace, ".", Some(DOTNET_OUTPUT_PATTERN))?;
    workspace.close()
}

#[test]
#[ignore]
fn shows_latest_in_directory_with_csproj() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    let csproj = make_csproj_with_tfm("TargetFramework", "netstandard2.0");
    touch_path(&workspace, "project.csproj", Some(&csproj))?;
    expect_output(&workspace, ".", Some(DOTNET_OUTPUT_PATTERN))?;
    expect_output(&workspace, ".", Some(DOTNET_TFM_PATTERN))?;
    workspace.close()
}

#[test]
#[ignore]
fn shows_latest_in_directory_with_fsproj() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    touch_path(&workspace, "project.fsproj", None)?;
    expect_output(&workspace, ".", Some(DOTNET_OUTPUT_PATTERN))?;
    workspace.close()
}

#[test]
#[ignore]
fn shows_latest_in_directory_with_xproj() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    touch_path(&workspace, "project.xproj", None)?;
    expect_output(&workspace, ".", Some(DOTNET_OUTPUT_PATTERN))?;
    workspace.close()
}

#[test]
#[ignore]
fn shows_latest_in_directory_with_project_json() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    touch_path(&workspace, "project.json", None)?;
    expect_output(&workspace, ".", Some(DOTNET_OUTPUT_PATTERN))?;
    workspace.close()
}

#[test]
#[ignore]
fn shows_pinned_in_directory_with_global_json() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    let global_json = make_pinned_sdk_json(DOTNET_PINNED_VERSION);
    touch_path(&workspace, "global.json", Some(&global_json))?;
    expect_output(&workspace, ".", Some(DOTNET_PINNED_VERSION_OUTPUT_PATTERN))?;
    workspace.close()
}

#[test]
#[ignore]
fn shows_pinned_in_project_below_root_with_global_json() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    let global_json = make_pinned_sdk_json(DOTNET_PINNED_VERSION);
    let csproj = make_csproj_with_tfm("TargetFramework", DOTNET_TFM_PINNED_VERSION);
    touch_path(&workspace, "global.json", Some(&global_json))?;
    touch_path(&workspace, "project/project.csproj", Some(&csproj))?;
    expect_output(
        &workspace,
        "project",
        Some(DOTNET_PINNED_VERSION_OUTPUT_PATTERN),
    )?;
    workspace.close()
}

#[test]
#[ignore]
fn shows_pinned_in_deeply_nested_project_within_repository() -> io::Result<()> {
    let workspace = create_workspace(true)?;
    let global_json = make_pinned_sdk_json("1.2.3");
    let csproj = make_csproj_with_tfm("TargetFramework", DOTNET_TFM_PINNED_VERSION);
    touch_path(&workspace, "global.json", Some(&global_json))?;
    touch_path(
        &workspace,
        "deep/path/to/project/project.csproj",
        Some(&csproj),
    )?;
    expect_output(
        &workspace,
        "deep/path/to/project",
        Some(DOTNET_PINNED_VERSION_OUTPUT_PATTERN),
    )?;
    workspace.close()
}

#[test]
#[ignore]
fn shows_single_tfm() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    let csproj = make_csproj_with_tfm("TargetFramework", "netstandard2.0");
    touch_path(&workspace, "project.csproj", Some(&csproj))?;
    expect_output(&workspace, ".", Some("•NET v2.2.402"))?;
    expect_output(&workspace, ".", Some("🎯 netstandard2.0"))?;
    workspace.close()
}

#[test]
#[ignore]
fn shows_multiple_tfms() -> io::Result<()> {
    let workspace = create_workspace(false)?;
    let csproj = make_csproj_with_tfm("TargetFrameworks", "netstandard2.0;net461");
    touch_path(&workspace, "project.csproj", Some(&csproj))?;
    expect_output(&workspace, ".", Some("•NET v2.2.402"))?;
    expect_output(&workspace, ".", Some("🎯 netstandard2.0;net461"))?;
    workspace.close()
}

fn create_workspace(is_repo: bool) -> io::Result<TempDir> {
    let repo_dir = tempfile::tempdir()?;

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
    write!(file, "{}", contents.unwrap_or(""))?;
    file.sync_data()
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

fn make_csproj_with_tfm(tfm_element: &str, tfm: &str) -> String {
    let json_text = r#"
        <Project>
            <PropertyGroup>
                <TFM_ELEMENT>TFM_VALUE</TFM_ELEMENT>
            </PropertyGroup>
        </Project>
    "#;
    json_text
        .replace("TFM_ELEMENT", tfm_element)
        .replace("TFM_VALUE", tfm)
}

fn expect_output(workspace: &TempDir, run_from: &str, pattern: Option<&str>) -> io::Result<()> {
    let run_path = workspace.path().join(run_from);
    let output = common::render_module("dotnet")
        .current_dir(run_path)
        .output()?;
    let text = String::from_utf8(output.stdout).unwrap();

    // This can be helpful for debugging
    eprintln!("The dotnet module showed: {}", text);

    match pattern {
        Some(pattern) => {
            let re = Regex::new(pattern).unwrap();
            assert!(re.is_match(&text));
        }
        None => assert!(text.is_empty()),
    }

    Ok(())
}
