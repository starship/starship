use quick_xml::events::Event;
use quick_xml::Reader;
use std::ffi::OsStr;
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use std::str;

use super::{Context, Module, ModuleConfig};
use crate::configs::dotnet::DotnetConfig;
use crate::formatter::StringFormatter;
use crate::utils;

type JValue = serde_json::Value;
use crate::formatter::VersionFormatter;

const GLOBAL_JSON_FILE: &str = "global.json";
const PROJECT_JSON_FILE: &str = "project.json";

/// A module which shows the latest (or pinned) version of the dotnet SDK

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("dotnet");
    let config = DotnetConfig::try_load(module.config);

    // First check if this is a DotNet Project before doing the O(n)
    // check for the version using the JSON files
    let is_dotnet_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_dotnet_project {
        return None;
    }

    let dotnet_files = get_local_dotnet_files(context).ok()?;

    // Internally, this module uses its own mechanism for version detection.
    // Typically it is twice as fast as running `dotnet --version`.
    let enable_heuristic = config.heuristic;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "symbol" => Some(Ok(config.symbol)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => {
                    let version = if enable_heuristic {
                        let repo_root = context.get_repo().ok().and_then(|r| r.workdir.as_deref());
                        estimate_dotnet_version(
                            context,
                            &dotnet_files,
                            &context.current_dir,
                            repo_root,
                        )
                    } else {
                        get_version_from_cli(context)
                    };
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &version?,
                        config.version_format,
                    )
                    .map(Ok)
                }
                "tfm" => find_current_tfm(&dotnet_files).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `dotnet`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn find_current_tfm(files: &[DotNetFile]) -> Option<String> {
    let get_file_of_type = |t: FileType| files.iter().find(|f| f.file_type == t);

    let relevant_file = get_file_of_type(FileType::ProjectFile)?;

    get_tfm_from_project_file(relevant_file.path.as_path())
}

fn get_tfm_from_project_file(path: &Path) -> Option<String> {
    let project_file = utils::read_file(path).ok()?;
    let mut reader = Reader::from_str(&project_file);
    reader.trim_text(true);

    let mut in_tfm = false;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            // for triggering namespaced events, use this instead:
            // match reader.read_namespaced_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                // for namespaced:
                // Ok((ref namespace_value, Event::Start(ref e)))
                match e.name().as_ref() {
                    b"TargetFrameworks" => in_tfm = true,
                    b"TargetFramework" => in_tfm = true,
                    _ => in_tfm = false,
                }
            }
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => {
                if in_tfm {
                    return e.unescape().ok().map(std::borrow::Cow::into_owned);
                }
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => {
                log::error!(
                    "Error parsing project file {path:?} at position {pos}: {e:?}",
                    pos = reader.buffer_position()
                );
                return None;
            }
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    None
}

fn estimate_dotnet_version(
    context: &Context,
    files: &[DotNetFile],
    current_dir: &Path,
    repo_root: Option<&Path>,
) -> Option<String> {
    let get_file_of_type = |t: FileType| files.iter().find(|f| f.file_type == t);

    // It's important to check for a global.json or a solution file first,
    // but otherwise we can take any relevant file. We'll take whichever is first.
    let relevant_file = get_file_of_type(FileType::GlobalJson)
        .or_else(|| get_file_of_type(FileType::SolutionFile))
        .or_else(|| files.iter().next())?;

    match relevant_file.file_type {
        FileType::GlobalJson => get_pinned_sdk_version_from_file(relevant_file.path.as_path())
            .or_else(|| get_latest_sdk_from_cli(context)),
        FileType::SolutionFile => {
            // With this heuristic, we'll assume that a "global.json" won't
            // be found in any directory above the solution file.
            get_latest_sdk_from_cli(context)
        }
        _ => {
            // If we see a dotnet project, we'll check a small number of neighboring
            // directories to see if we can find a global.json. Otherwise, assume the
            // latest SDK is in use.
            try_find_nearby_global_json(current_dir, repo_root)
                .or_else(|| get_latest_sdk_from_cli(context))
        }
    }
}

/// Looks for a `global.json` which may exist in one of the parent directories of the current path.
/// If there is one present, and it contains valid version pinning information, then return that version.
///
/// The following places are scanned:
///     - The parent of the current directory
///       (Unless there is a git repository, and the parent is above the root of that repository)
///     - The root of the git repository
///       (If there is one)
fn try_find_nearby_global_json(current_dir: &Path, repo_root: Option<&Path>) -> Option<String> {
    let current_dir_is_repo_root = repo_root.map_or(false, |r| r == current_dir);
    let parent_dir = if current_dir_is_repo_root {
        // Don't scan the parent directory if it's above the root of a git repository
        None
    } else {
        current_dir.parent()
    };

    // Check the parent directory, or otherwise the repository root, for a global.json
    let mut check_dirs = parent_dir
        .iter()
        .chain(&repo_root)
        .copied() // Copies the reference, not the Path itself
        .collect::<Vec<&Path>>();

    // The parent directory and repository root may be the same directory,
    // so avoid checking it twice.
    check_dirs.dedup();

    check_dirs
        .iter()
        // repo_root may be the same as the current directory. We don't need to scan it again.
        .filter(|&&d| d != current_dir)
        .find_map(|d| check_directory_for_global_json(d))
}

fn check_directory_for_global_json(path: &Path) -> Option<String> {
    let global_json_path = path.join(GLOBAL_JSON_FILE);
    log::debug!(
        "Checking if global.json exists at: {}",
        &global_json_path.display()
    );
    if global_json_path.exists() {
        get_pinned_sdk_version_from_file(&global_json_path)
    } else {
        None
    }
}

fn get_pinned_sdk_version_from_file(path: &Path) -> Option<String> {
    let json_text = crate::utils::read_file(path).ok()?;
    log::debug!(
        "Checking if .NET SDK version is pinned in: {}",
        path.display()
    );
    get_pinned_sdk_version(&json_text)
}

fn get_pinned_sdk_version(json: &str) -> Option<String> {
    let parsed_json: JValue = serde_json::from_str(json).ok()?;

    match parsed_json {
        JValue::Object(root) => {
            let sdk = root.get("sdk")?;
            match sdk {
                JValue::Object(sdk) => {
                    let version = sdk.get("version")?;
                    match version {
                        JValue::String(version_string) => {
                            let mut buffer = String::with_capacity(version_string.len() + 1);
                            buffer.push_str(version_string);
                            Some(buffer)
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

fn get_local_dotnet_files(context: &Context) -> Result<Vec<DotNetFile>, std::io::Error> {
    Ok(context
        .dir_contents()?
        .files()
        .filter_map(|p| {
            get_dotnet_file_type(p).map(|t| DotNetFile {
                path: context.current_dir.join(p),
                file_type: t,
            })
        })
        .collect())
}

fn get_dotnet_file_type(path: &Path) -> Option<FileType> {
    let file_name_lower = map_str_to_lower(path.file_name());

    match file_name_lower.as_ref().map(std::convert::AsRef::as_ref) {
        Some(GLOBAL_JSON_FILE) => return Some(FileType::GlobalJson),
        Some(PROJECT_JSON_FILE) => return Some(FileType::ProjectJson),
        _ => (),
    };

    let extension_lower = map_str_to_lower(path.extension());

    match extension_lower.as_ref().map(std::convert::AsRef::as_ref) {
        Some("sln") => return Some(FileType::SolutionFile),
        Some("csproj" | "fsproj" | "xproj") => return Some(FileType::ProjectFile),
        Some("props" | "targets") => return Some(FileType::MsBuildFile),
        _ => (),
    };

    None
}

fn map_str_to_lower(value: Option<&OsStr>) -> Option<String> {
    Some(value?.to_str()?.to_ascii_lowercase())
}

fn get_version_from_cli(context: &Context) -> Option<String> {
    let version_output = context.exec_cmd("dotnet", &["--version"])?;
    Some(format!("v{}", version_output.stdout.trim()))
}

fn get_latest_sdk_from_cli(context: &Context) -> Option<String> {
    match context.exec_cmd("dotnet", &["--list-sdks"]) {
        Some(sdks_output) => {
            fn parse_failed<T>() -> Option<T> {
                log::warn!("Unable to parse the output from `dotnet --list-sdks`.");
                None
            }
            let latest_sdk = sdks_output
                .stdout
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .last()
                .or_else(parse_failed)?;
            let take_until = latest_sdk.find('[').or_else(parse_failed)? - 1;
            if take_until > 1 {
                let version = &latest_sdk[..take_until];
                let mut buffer = String::with_capacity(version.len() + 1);
                buffer.push_str(version);
                Some(buffer)
            } else {
                parse_failed()
            }
        }
        None => {
            // Older versions of the dotnet cli do not support the --list-sdks command
            // So, if the status code indicates failure, fall back to `dotnet --version`
            log::debug!(
                "Received a non-success exit code from `dotnet --list-sdks`. \
                 Falling back to `dotnet --version`.",
            );
            get_version_from_cli(context)
        }
    }
}

struct DotNetFile {
    path: PathBuf,
    file_type: FileType,
}

#[derive(PartialEq)]
enum FileType {
    ProjectJson,
    ProjectFile,
    GlobalJson,
    SolutionFile,
    MsBuildFile,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use crate::utils::create_command;
    use nu_ansi_term::Color;
    use std::fs::{self, OpenOptions};
    use std::io::{self, Write};
    use tempfile::{self, TempDir};

    #[test]
    fn shows_nothing_in_directory_with_zero_relevant_files() -> io::Result<()> {
        let workspace = create_workspace(false)?;
        expect_output(workspace.path(), None);
        workspace.close()
    }

    #[test]
    fn shows_latest_in_directory_with_directory_build_props_file() -> io::Result<()> {
        let workspace = create_workspace(false)?;
        touch_path(&workspace, "Directory.Build.props", None)?;
        expect_output(
            workspace.path(),
            Some(format!(
                "via {}",
                Color::Blue.bold().paint(".NET v3.1.103 ")
            )),
        );
        workspace.close()
    }

    #[test]
    fn shows_latest_in_directory_with_directory_build_targets_file() -> io::Result<()> {
        let workspace = create_workspace(false)?;
        touch_path(&workspace, "Directory.Build.targets", None)?;
        expect_output(
            workspace.path(),
            Some(format!(
                "via {}",
                Color::Blue.bold().paint(".NET v3.1.103 ")
            )),
        );
        workspace.close()
    }

    #[test]
    fn shows_latest_in_directory_with_packages_props_file() -> io::Result<()> {
        let workspace = create_workspace(false)?;
        touch_path(&workspace, "Packages.props", None)?;
        expect_output(
            workspace.path(),
            Some(format!(
                "via {}",
                Color::Blue.bold().paint(".NET v3.1.103 ")
            )),
        );
        workspace.close()
    }

    #[test]
    fn shows_latest_in_directory_with_solution() -> io::Result<()> {
        let workspace = create_workspace(false)?;
        touch_path(&workspace, "solution.sln", None)?;
        expect_output(workspace.path(), None);
        workspace.close()
    }

    #[test]
    fn shows_latest_in_directory_with_csproj() -> io::Result<()> {
        let workspace = create_workspace(false)?;
        let csproj = make_csproj_with_tfm("TargetFramework", "netstandard2.0");
        touch_path(&workspace, "project.csproj", Some(&csproj))?;
        expect_output(
            workspace.path(),
            Some(format!(
                "via {}",
                Color::Blue.bold().paint(".NET v3.1.103 🎯 netstandard2.0 ")
            )),
        );
        workspace.close()
    }

    #[test]
    fn shows_latest_in_directory_with_fsproj() -> io::Result<()> {
        let workspace = create_workspace(false)?;
        touch_path(&workspace, "project.fsproj", None)?;
        expect_output(
            workspace.path(),
            Some(format!(
                "via {}",
                Color::Blue.bold().paint(".NET v3.1.103 ")
            )),
        );
        workspace.close()
    }

    #[test]
    fn shows_latest_in_directory_with_xproj() -> io::Result<()> {
        let workspace = create_workspace(false)?;
        touch_path(&workspace, "project.xproj", None)?;
        expect_output(
            workspace.path(),
            Some(format!(
                "via {}",
                Color::Blue.bold().paint(".NET v3.1.103 ")
            )),
        );
        workspace.close()
    }

    #[test]
    fn shows_latest_in_directory_with_project_json() -> io::Result<()> {
        let workspace = create_workspace(false)?;
        touch_path(&workspace, "project.json", None)?;
        expect_output(
            workspace.path(),
            Some(format!(
                "via {}",
                Color::Blue.bold().paint(".NET v3.1.103 ")
            )),
        );
        workspace.close()
    }

    #[test]
    fn shows_pinned_in_directory_with_global_json() -> io::Result<()> {
        let workspace = create_workspace(false)?;
        let global_json = make_pinned_sdk_json("1.2.3");
        touch_path(&workspace, "global.json", Some(&global_json))?;
        expect_output(
            workspace.path(),
            Some(format!("via {}", Color::Blue.bold().paint(".NET v1.2.3 "))),
        );
        workspace.close()
    }

    #[test]
    fn shows_pinned_in_project_below_root_with_global_json() -> io::Result<()> {
        let workspace = create_workspace(false)?;
        let global_json = make_pinned_sdk_json("1.2.3");
        let csproj = make_csproj_with_tfm("TargetFramework", "netstandard2.0");
        touch_path(&workspace, "global.json", Some(&global_json))?;
        touch_path(&workspace, "project/project.csproj", Some(&csproj))?;
        expect_output(
            &workspace.path().join("project"),
            Some(format!(
                "via {}",
                Color::Blue.bold().paint(".NET v1.2.3 🎯 netstandard2.0 ")
            )),
        );
        workspace.close()
    }

    #[test]
    fn shows_pinned_in_deeply_nested_project_within_repository() -> io::Result<()> {
        let workspace = create_workspace(true)?;
        let global_json = make_pinned_sdk_json("1.2.3");
        let csproj = make_csproj_with_tfm("TargetFramework", "netstandard2.0");
        touch_path(&workspace, "global.json", Some(&global_json))?;
        touch_path(
            &workspace,
            "deep/path/to/project/project.csproj",
            Some(&csproj),
        )?;
        expect_output(
            &workspace.path().join("deep/path/to/project"),
            Some(format!(
                "via {}",
                Color::Blue.bold().paint(".NET v1.2.3 🎯 netstandard2.0 ")
            )),
        );
        workspace.close()
    }

    #[test]
    fn shows_single_tfm() -> io::Result<()> {
        let workspace = create_workspace(false)?;
        let csproj = make_csproj_with_tfm("TargetFramework", "netstandard2.0");
        touch_path(&workspace, "project.csproj", Some(&csproj))?;
        expect_output(
            workspace.path(),
            Some(format!(
                "via {}",
                Color::Blue.bold().paint(".NET v3.1.103 🎯 netstandard2.0 ")
            )),
        );
        workspace.close()
    }

    #[test]
    fn shows_multiple_tfms() -> io::Result<()> {
        let workspace = create_workspace(false)?;
        let csproj = make_csproj_with_tfm("TargetFrameworks", "netstandard2.0;net461");
        touch_path(&workspace, "project.csproj", Some(&csproj))?;
        expect_output(
            workspace.path(),
            Some(format!(
                "via {}",
                Color::Blue
                    .bold()
                    .paint(".NET v3.1.103 🎯 netstandard2.0;net461 ")
            )),
        );
        workspace.close()
    }

    fn create_workspace(is_repo: bool) -> io::Result<TempDir> {
        let repo_dir = tempfile::tempdir()?;

        if is_repo {
            create_command("git")?
                .args(["init", "--quiet"])
                .current_dir(repo_dir.path())
                .output()?;
        }

        Ok(repo_dir)
    }

    fn touch_path(
        workspace: &TempDir,
        relative_path: &str,
        contents: Option<&str>,
    ) -> io::Result<()> {
        let path = workspace.path().join(relative_path);

        fs::create_dir_all(
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

    fn expect_output(dir: &Path, expected: Option<String>) {
        let actual = ModuleRenderer::new("dotnet").path(dir).collect();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_version_from_global_json() {
        let json_text = r#"
        {
            "sdk": {
                "version": "1.2.3"
            }
        }
    "#;

        let version = get_pinned_sdk_version(json_text).unwrap();
        assert_eq!("1.2.3", version);
    }

    #[test]
    fn should_ignore_empty_global_json() {
        let json_text = "{}";

        let version = get_pinned_sdk_version(json_text);
        assert!(version.is_none());
    }
}
