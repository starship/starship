use quick_xml::events::Event;
use quick_xml::Reader;
use std::ffi::OsStr;
use std::iter::Iterator;
use std::ops::Deref;
use std::path::Path;
use std::str;

use super::{Context, Module, RootModuleConfig};
use crate::configs::dotnet::DotnetConfig;
use crate::formatter::StringFormatter;
use crate::utils;

type JValue = serde_json::Value;

const GLOBAL_JSON_FILE: &str = "global.json";
const PROJECT_JSON_FILE: &str = "project.json";
const DIRECTORY_BUILD_PROPS_FILE: &str = "Directory.Build.props";
const DIRECTORY_BUILD_TARGETS_FILE: &str = "Directory.Build.targets";
const PACKAGES_PROPS_FILE: &str = "Packages.props";

/// A module which shows the latest (or pinned) version of the dotnet SDK
///
/// Will display if any of the following files are present in
/// the current directory:
/// global.json, project.json, *.sln, *.csproj, *.fsproj, *.xproj
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    // First check if this is a DotNet Project before doing the O(n)
    // check for the version using the JSON files
    let is_dotnet_project = context
        .try_begin_scan()?
        .set_files(&[
            GLOBAL_JSON_FILE,
            PROJECT_JSON_FILE,
            DIRECTORY_BUILD_PROPS_FILE,
            DIRECTORY_BUILD_TARGETS_FILE,
            PACKAGES_PROPS_FILE,
        ])
        .set_extensions(&["sln", "csproj", "fsproj", "xproj"])
        .is_match();

    if !is_dotnet_project {
        return None;
    }

    let dotnet_files = get_local_dotnet_files(context).ok()?;

    let mut module = context.new_module("dotnet");
    let config = DotnetConfig::try_load(module.config);

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
                        let repo_root = context.get_repo().ok().and_then(|r| r.root.as_deref());
                        estimate_dotnet_version(&dotnet_files, &context.current_dir, repo_root)
                    } else {
                        get_version_from_cli()
                    };
                    version.map(|v| Ok(v.0))
                }
                "tfm" => find_current_tfm(&dotnet_files).map(Ok),
                _ => None,
            })
            .parse(None)
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

fn find_current_tfm<'a>(files: &[DotNetFile<'a>]) -> Option<String> {
    let get_file_of_type = |t: FileType| files.iter().find(|f| f.file_type == t);

    let relevant_file = get_file_of_type(FileType::ProjectFile)?;

    get_tfm_from_project_file(relevant_file.path)
}

fn get_tfm_from_project_file(path: &Path) -> Option<String> {
    let project_file = utils::read_file(path).ok()?;
    let mut reader = Reader::from_str(&project_file);
    reader.trim_text(true);

    let mut in_tfm = false;
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            // for triggering namespaced events, use this instead:
            // match reader.read_namespaced_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                // for namespaced:
                // Ok((ref namespace_value, Event::Start(ref e)))
                match e.name() {
                    b"TargetFrameworks" => in_tfm = true,
                    b"TargetFramework" => in_tfm = true,
                    _ => in_tfm = false,
                }
            }
            // unescape and decode the text event using the reader encoding
            Ok(Event::Text(e)) => {
                if in_tfm {
                    return e.unescape_and_decode(&reader).ok();
                }
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    None
}

fn estimate_dotnet_version<'a>(
    files: &[DotNetFile<'a>],
    current_dir: &Path,
    repo_root: Option<&Path>,
) -> Option<Version> {
    let get_file_of_type = |t: FileType| files.iter().find(|f| f.file_type == t);

    // It's important to check for a global.json or a solution file first,
    // but otherwise we can take any relevant file. We'll take whichever is first.
    let relevant_file = get_file_of_type(FileType::GlobalJson)
        .or_else(|| get_file_of_type(FileType::SolutionFile))
        .or_else(|| files.iter().next())?;

    match relevant_file.file_type {
        FileType::GlobalJson => {
            get_pinned_sdk_version_from_file(relevant_file.path).or_else(get_latest_sdk_from_cli)
        }
        FileType::SolutionFile => {
            // With this heuristic, we'll assume that a "global.json" won't
            // be found in any directory above the solution file.
            get_latest_sdk_from_cli()
        }
        _ => {
            // If we see a dotnet project, we'll check a small number of neighboring
            // directories to see if we can find a global.json. Otherwise, assume the
            // latest SDK is in use.
            try_find_nearby_global_json(current_dir, repo_root).or_else(get_latest_sdk_from_cli)
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
fn try_find_nearby_global_json(current_dir: &Path, repo_root: Option<&Path>) -> Option<Version> {
    let current_dir_is_repo_root = repo_root.map(|r| r == current_dir).unwrap_or(false);
    let parent_dir = if current_dir_is_repo_root {
        // Don't scan the parent directory if it's above the root of a git repository
        None
    } else {
        current_dir.parent()
    };

    // Check the parent directory, or otherwise the repository root, for a global.json
    let mut check_dirs = parent_dir
        .iter()
        .chain(repo_root.iter())
        .copied() // Copies the reference, not the Path itself
        .collect::<Vec<&Path>>();

    // The parent directory and repository root may be the same directory,
    // so avoid checking it twice.
    check_dirs.dedup();

    check_dirs
        .iter()
        // repo_root may be the same as the current directory. We don't need to scan it again.
        .filter(|&&d| d != current_dir)
        .filter_map(|d| check_directory_for_global_json(d))
        // This will lazily evaluate the first directory with a global.json
        .next()
}

fn check_directory_for_global_json(path: &Path) -> Option<Version> {
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

fn get_pinned_sdk_version_from_file(path: &Path) -> Option<Version> {
    let json_text = crate::utils::read_file(path).ok()?;
    log::debug!(
        "Checking if .NET SDK version is pinned in: {}",
        path.display()
    );
    get_pinned_sdk_version(&json_text)
}

fn get_pinned_sdk_version(json: &str) -> Option<Version> {
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
                            buffer.push('v');
                            buffer.push_str(version_string);
                            Some(Version(buffer))
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

fn get_local_dotnet_files<'a>(context: &'a Context) -> Result<Vec<DotNetFile<'a>>, std::io::Error> {
    Ok(context
        .dir_contents()?
        .files()
        .filter_map(|p| {
            get_dotnet_file_type(p).map(|t| DotNetFile {
                path: p.as_ref(),
                file_type: t,
            })
        })
        .collect())
}

fn get_dotnet_file_type(path: &Path) -> Option<FileType> {
    let file_name_lower = map_str_to_lower(path.file_name());

    match file_name_lower.as_ref().map(|f| f.as_ref()) {
        Some(GLOBAL_JSON_FILE) => return Some(FileType::GlobalJson),
        Some(PROJECT_JSON_FILE) => return Some(FileType::ProjectJson),
        _ => (),
    };

    let extension_lower = map_str_to_lower(path.extension());

    match extension_lower.as_ref().map(|f| f.as_ref()) {
        Some("sln") => return Some(FileType::SolutionFile),
        Some("csproj") | Some("fsproj") | Some("xproj") => return Some(FileType::ProjectFile),
        Some("props") | Some("targets") => return Some(FileType::MsBuildFile),
        _ => (),
    };

    None
}

fn map_str_to_lower(value: Option<&OsStr>) -> Option<String> {
    Some(value?.to_str()?.to_ascii_lowercase())
}

fn get_version_from_cli() -> Option<Version> {
    let version_output = utils::exec_cmd("dotnet", &["--version"])?;
    Some(Version(format!("v{}", version_output.stdout.trim())))
}

fn get_latest_sdk_from_cli() -> Option<Version> {
    match utils::exec_cmd("dotnet", &["--list-sdks"]) {
        Some(sdks_output) => {
            fn parse_failed<T>() -> Option<T> {
                log::warn!("Unable to parse the output from `dotnet --list-sdks`.");
                None
            };
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
                buffer.push('v');
                buffer.push_str(version);
                Some(Version(buffer))
            } else {
                parse_failed()
            }
        }
        None => {
            // Older versions of the dotnet cli do not support the --list-sdks command
            // So, if the status code indicates failure, fall back to `dotnet --version`
            log::warn!(
                "Received a non-success exit code from `dotnet --list-sdks`. \
                 Falling back to `dotnet --version`.",
            );
            get_version_from_cli()
        }
    }
}

struct DotNetFile<'a> {
    path: &'a Path,
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

struct Version(String);

impl Deref for Version {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
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
    assert_eq!("v1.2.3", version.0);
}

#[test]
fn should_ignore_empty_global_json() {
    let json_text = "{}";

    let version = get_pinned_sdk_version(json_text);
    assert!(version.is_none());
}
