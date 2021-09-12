#![warn(missing_docs)]
use sha1::{Digest, Sha1};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use yaml_rust::{Yaml, YamlLoader};

use super::{Context, Module, RootModuleConfig};
use crate::configs::pulumi::PulumiConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

static PULUMI_HOME: &str = "PULUMI_HOME";

/// Creates a module with the current Pulumi version and stack name.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("pulumi");
    let config = PulumiConfig::try_load(module.config);

    let project_file = find_package_file(&context.logical_dir)?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => {
                    let stdout = context.exec_cmd("pulumi", &["version"])?.stdout;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        parse_version(&stdout),
                        config.version_format,
                    )
                }
                .map(Ok),
                "stack" => stack_name(&project_file, context).map(Ok),
                _ => None,
            })
            .parse(None)
    });

    match parsed {
        Ok(x) => {
            module.set_segments(x);
            Some(module)
        }
        Err(e) => {
            log::warn!("Error in module `pulumi`:\n{}", e);
            None
        }
    }
}

/// Parse the output of `pulumi version` into just the version string.
///
/// Normally, this just means returning it. When Pulumi is being developed, it
/// can return results like `3.12.0-alpha.1630554544+f89e9a29.dirty`, which we
/// don't want to see. Instead we display that as `3.12.0-alpha`.
fn parse_version(version: &str) -> &str {
    &version[0..version
        .trim()
        .split('.')
        .take(3)
        .map(&str::len)
        .sum::<usize>()
        + 2]
}

/// Find a file describing a Pulumi package in the current directory (or any parrent directory).
fn find_package_file(path: &Path) -> Option<PathBuf> {
    for path in path.ancestors() {
        log::trace!("Looking for package file in {:?}", path);
        let dir = std::fs::read_dir(path).ok()?;
        let goal = dir.filter_map(Result::ok).find(|path| {
            path.file_name() == OsStr::new("Pulumi.yaml")
                || path.file_name() == OsStr::new("Pulumi.yml")
        });
        if let Some(goal) = goal {
            return Some(goal.path());
        }
    }
    log::trace!("Did not find a Pulumi package file");
    None
}

/// We get the name of the current stack.
///
/// Pulumi has no CLI option that is fast enough to get this for us, but finding
/// the location is simple. We get it ourselves.
fn stack_name(project_file: &Path, context: &Context) -> Option<String> {
    let mut file = File::open(&project_file).ok()?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    let name = YamlLoader::load_from_str(&mut contents).ok().map(
        |mut yaml| -> Option<Option<String>> {
            log::trace!("Parsed {:?} into yaml", project_file);
            let yaml = yaml.swap_remove(0);
            yaml.into_hash().map(|mut hash| -> Option<String> {
                hash.remove(&Yaml::String("name".to_string()))?
                    .into_string()
            })
        },
    )???;
    log::trace!("Found project name: {:?}", name);

    let workspace_file = get_pulumi_workspace(context, &name, project_file)
        .map(File::open)?
        .ok()?;
    log::trace!("Trying to read workspace_file: {:?}", workspace_file);
    let workspace: serde_json::Value = match serde_json::from_reader(workspace_file) {
        Ok(k) => k,
        Err(e) => {
            log::debug!("Failed to parse workspace file: {}", e);
            return None;
        }
    };
    log::trace!("Read workspace_file: {:?}", workspace);
    workspace
        .as_object()?
        .get("stack")?
        .as_str()
        .map(ToString::to_string)
}

/// Calculates the path of the workspace settings file for a given pulumi stack.
fn get_pulumi_workspace(context: &Context, name: &str, project_file: &Path) -> Option<PathBuf> {
    let project_file = if cfg!(test) {
        // Because this depends on the absolute path of the file, it changes in
        // each test run. We thus mock it.
        "test".to_string()
    } else {
        let mut hasher = Sha1::new();
        hasher.update(project_file.to_str()?.as_bytes());
        encode_to_hex(&hasher.finalize().to_vec())
    };
    let unique_file_name = format!("{}-{}-workspace.json", name, project_file);
    let mut path = pulumi_home_dir(context)?;
    path.push("workspaces");
    path.push(unique_file_name);
    Some(path)
}

const HEXTABLE: &[char] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Encode a u8 slice into a hexadecimal string.
fn encode_to_hex(slice: &[u8]) -> String {
    // let mut j = 0;
    let mut dst = Vec::with_capacity(slice.len() * 2);
    for &v in slice {
        dst.push(HEXTABLE[(v >> 4) as usize] as u8);
        dst.push(HEXTABLE[(v & 0x0f) as usize] as u8);
    }
    String::from_utf8(dst).unwrap()
}

/// Get the Pulumi home directory. We first check `PULUMI_HOME`. If that isn't
/// set, we return `$HOME/.pulumi`.
fn pulumi_home_dir(context: &Context) -> Option<PathBuf> {
    if let Some(k) = context.get_env(PULUMI_HOME) {
        std::path::PathBuf::from_str(&k).ok()
    } else {
        context.get_home().map(|p| p.join(".pulumi"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use clap::ArgMatches;

    #[test]
    fn pulumi_version_release() {
        let input = "3.12.0";
        assert_eq!(parse_version(input), input);
    }

    #[test]
    fn pulumi_version_prerelease() {
        let input = "3.12.0-alpha";
        assert_eq!(parse_version(input), input);
    }

    #[test]
    fn pulumi_version_dirty() {
        let input = "3.12.0-alpha.1630554544+f89e9a29.dirty";
        assert_eq!(parse_version(input), "3.12.0-alpha");
    }

    #[test]
    fn sha1_hex() {
        assert_eq!(
            encode_to_hex(&[8, 13, 9, 189, 129, 94]),
            "080d09bd815e".to_string()
        );
    }

    #[test]
    fn get_home_dir() {
        let mut context = Context::new(ArgMatches::default());
        context.env.insert("HOME", "/home/sweet/home".to_string());
        assert_eq!(
            pulumi_home_dir(&context),
            Some(PathBuf::from("/home/sweet/home/.pulumi"))
        );
        context.env.insert("PULUMI_HOME", "/a/dir".to_string());
        assert_eq!(pulumi_home_dir(&context), Some(PathBuf::from("/a/dir")))
    }

    #[test]
    fn test_get_pulumi_workspace() {
        let mut context = Context::new(ArgMatches::default());
        context.env.insert("HOME", "/home/sweet/home".to_string());
        let name = "foobar";
        let project_file = PathBuf::from("/hello/Pulumi.yaml");
        assert_eq!(
            get_pulumi_workspace(&context, name, &project_file),
            Some("/home/sweet/home/.pulumi/workspaces/foobar-test-workspace.json")
                .map(PathBuf::from)
        );
    }

    #[test]
    #[ignore]
    /// This test confirms a full render. This means finding a Pulumi.yml file,
    /// tracing back to the backing workspace settings file, and printing the
    /// stack name.
    ///
    /// To do this, we need to create these files.
    fn test_render_valid_paths() -> std::io::Result<()> {
        use std::io::Write;
        let dir = tempfile::tempdir()?;
        let root = std::fs::canonicalize(dir.into_path()).unwrap();
        let mut yaml = File::create(root.join("Pulumi.yml"))?;
        yaml.write_all("name: starship\nruntime: nodejs\ndescription: A thing\n".as_bytes())?;
        yaml.sync_all()?;

        let workspace_path = root.join(".pulumi").join("workspaces");
        let _ = std::fs::create_dir_all(&workspace_path)?;
        let workspace_path = &workspace_path.join("starship-test-workspace.json");
        let mut workspace = File::create(&workspace_path)?;
        serde_json::to_writer_pretty(
            &mut workspace,
            &serde_json::json!(
                {
                    "stack": "launch"
                }
            ),
        )?;
        workspace.sync_all()?;
        let rendered = ModuleRenderer::new("pulumi")
            .path(root.clone())
            .logical_path(root.clone())
            .config(toml::toml! {
                [pulumi]
                format = "in [$symbol($stack)]($style) "
            })
            .env("HOME", root.to_str().unwrap())
            .collect();
        let expected = format!("in {} ", Color::Fixed(5).bold().paint("🚀 launch"));
        assert_eq!(expected, rendered.expect("a result"));
        Ok(())
    }
}
