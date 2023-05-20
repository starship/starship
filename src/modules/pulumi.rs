#![warn(missing_docs)]
use serde::Deserialize;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use yaml_rust::{Yaml, YamlLoader};

use super::{Context, Module, ModuleConfig};
use crate::configs::pulumi::PulumiConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

static PULUMI_HOME: &str = "PULUMI_HOME";

#[derive(Deserialize)]
struct Credentials {
    current: Option<String>,
    accounts: Option<HashMap<String, Account>>,
}

#[derive(Deserialize)]
struct Account {
    username: Option<String>,
}

/// Creates a module with the current Pulumi version and stack name.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("pulumi");
    let config = PulumiConfig::try_load(module.config);

    let project_file_in_cwd = context
        .try_begin_scan()?
        .set_files(&["Pulumi.yaml", "Pulumi.yml"])
        .is_match();

    if !project_file_in_cwd && !config.search_upwards {
        return None;
    }

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
                "username" => get_pulumi_username(context).map(Ok),
                "stack" => stack_name(&project_file, context).map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
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

/// Parse and sanitize the output of `pulumi version` into just the version string.
///
/// Normally, this just means returning it. When Pulumi is being developed, it
/// can return results like `3.12.0-alpha.1630554544+f89e9a29.dirty`, which we
/// don't want to see. Instead we display that as `3.12.0-alpha`.
fn parse_version(version: &str) -> &str {
    let new_version = version.strip_prefix('v').unwrap_or(version);

    let sanitized_version = new_version.trim_end();

    let mut periods = 0;
    for (i, c) in sanitized_version.as_bytes().iter().enumerate() {
        if *c == b'.' {
            if periods == 2 {
                return &sanitized_version[0..i];
            } else {
                periods += 1;
            }
        }
    }
    // We didn't hit 3 periods, so we just return the whole string.
    sanitized_version
}

/// Find a file describing a Pulumi package in the current directory (or any parent directory).
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
    let mut file = File::open(project_file).ok()?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    let name = YamlLoader::load_from_str(&contents).ok().and_then(
        |yaml| -> Option<Option<String>> {
            log::trace!("Parsed {:?} into yaml", project_file);
            let yaml = yaml.into_iter().next()?;
            yaml.into_hash().map(|mut hash| -> Option<String> {
                hash.remove(&Yaml::String("name".to_string()))?
                    .into_string()
            })
        },
    )??;
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
        crate::utils::encode_to_hex(&hasher.finalize())
    };
    let unique_file_name = format!("{name}-{project_file}-workspace.json");
    let mut path = pulumi_home_dir(context)?;
    path.push("workspaces");
    path.push(unique_file_name);
    Some(path)
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

fn get_pulumi_username(context: &Context) -> Option<String> {
    let home_dir = pulumi_home_dir(context)?;
    let creds_path = home_dir.join("credentials.json");

    let file = File::open(creds_path).ok()?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let creds: Credentials = serde_json::from_reader(reader).ok()?;

    let current_api_provider = creds.current?;

    creds.accounts?.remove(&current_api_provider)?.username
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;
    use crate::context::Target;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn pulumi_version_release() {
        let expected = "3.12.0";
        let inputs: [&str; 6] = [
            "v3.12.0\r\n",
            "v3.12.0\n",
            "v3.12.0",
            "3.12.0\r\n",
            "3.12.0\n",
            "3.12.0",
        ];

        for input in inputs.iter() {
            assert_eq!(parse_version(input), expected);
        }
    }

    #[test]
    fn pulumi_version_prerelease() {
        let expected = "3.12.0-alpha";
        let inputs: [&str; 6] = [
            "v3.12.0-alpha\r\n",
            "v3.12.0-alpha\n",
            "v3.12.0-alpha",
            "3.12.0-alpha\r\n",
            "3.12.0-alpha\n",
            "3.12.0-alpha",
        ];

        for input in inputs.iter() {
            assert_eq!(parse_version(input), expected);
        }
    }

    #[test]
    fn pulumi_version_dirty() {
        let expected = "3.12.0-alpha";
        let inputs: [&str; 6] = [
            "v3.12.0-alpha.1630554544+f89e9a29.dirty\r\n",
            "v3.12.0-alpha.1630554544+f89e9a29.dirty\n",
            "v3.12.0-alpha.1630554544+f89e9a29.dirty",
            "3.12.0-alpha.1630554544+f89e9a29.dirty\r\n",
            "3.12.0-alpha.1630554544+f89e9a29.dirty\n",
            "3.12.0-alpha.1630554544+f89e9a29.dirty",
        ];

        for input in inputs.iter() {
            assert_eq!(parse_version(input), expected);
        }
    }

    #[test]
    fn get_home_dir() {
        let mut context = Context::new(Default::default(), Target::Main);
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
        let mut context = Context::new(Default::default(), Target::Main);
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
    fn version_render() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let pulumi_file = File::create(dir.path().join("Pulumi.yaml"))?;
        pulumi_file.sync_all()?;
        let rendered = ModuleRenderer::new("pulumi")
            .path(dir.path())
            .config(toml::toml! {
                [pulumi]
                format = "with [$version]($style) "
            })
            .collect();
        dir.close()?;
        let expected = format!("with {} ", Color::Fixed(5).bold().paint("v1.2.3-ver"));

        assert_eq!(expected, rendered.expect("a result"));
        Ok(())
    }

    #[test]
    /// This test confirms a full render. This means finding a Pulumi.yml file,
    /// tracing back to the backing workspace settings file, and printing the
    /// stack name.
    fn render_valid_paths() -> io::Result<()> {
        use io::Write;
        let dir = tempfile::tempdir()?;
        let root = dunce::canonicalize(dir.path())?;
        let mut yaml = File::create(root.join("Pulumi.yml"))?;
        yaml.write_all("name: starship\nruntime: nodejs\ndescription: A thing\n".as_bytes())?;
        yaml.sync_all()?;

        let workspace_path = root.join(".pulumi").join("workspaces");
        std::fs::create_dir_all(&workspace_path)?;
        let workspace_path = &workspace_path.join("starship-test-workspace.json");
        let mut workspace = File::create(workspace_path)?;
        serde_json::to_writer_pretty(
            &mut workspace,
            &serde_json::json!(
                {
                    "stack": "launch"
                }
            ),
        )?;
        workspace.sync_all()?;

        let credential_path = root.join(".pulumi");
        std::fs::create_dir_all(&credential_path)?;
        let credential_path = &credential_path.join("credentials.json");
        let mut credential = File::create(credential_path)?;
        serde_json::to_writer_pretty(
            &mut credential,
            &serde_json::json!(
                {
                    "current": "https://api.example.com",
                    "accessTokens": {
                        "https://api.example.com": "redacted",
                        "https://api.pulumi.com": "redacted"
                    },
                    "accounts": {
                        "https://api.example.com": {
                            "accessToken": "redacted",
                            "username": "test-user",
                            "lastValidatedAt": "2022-01-12T00:00:00.000000000-08:00"
                        }
                    }
                }
            ),
        )?;
        credential.sync_all()?;
        let rendered = ModuleRenderer::new("pulumi")
            .path(root.clone())
            .logical_path(root.clone())
            .config(toml::toml! {
                [pulumi]
                format = "via [$symbol($username@)$stack]($style) "
            })
            .env("HOME", root.to_str().unwrap())
            .collect();
        let expected = format!(
            "via {} ",
            Color::Fixed(5).bold().paint(" test-user@launch")
        );
        assert_eq!(expected, rendered.expect("a result"));
        dir.close()?;
        Ok(())
    }

    #[test]
    /// This test confirms a render when the account information incomplete, i.e.: no username for
    /// the current API.
    fn partial_login() -> io::Result<()> {
        use io::Write;
        let dir = tempfile::tempdir()?;
        let root = dunce::canonicalize(dir.path())?;
        let mut yaml = File::create(root.join("Pulumi.yml"))?;
        yaml.write_all("name: starship\nruntime: nodejs\ndescription: A thing\n".as_bytes())?;
        yaml.sync_all()?;

        let workspace_path = root.join(".pulumi").join("workspaces");
        std::fs::create_dir_all(&workspace_path)?;
        let workspace_path = &workspace_path.join("starship-test-workspace.json");
        let mut workspace = File::create(workspace_path)?;
        serde_json::to_writer_pretty(
            &mut workspace,
            &serde_json::json!(
                {
                    "stack": "launch"
                }
            ),
        )?;
        workspace.sync_all()?;

        let credential_path = root.join(".pulumi");
        std::fs::create_dir_all(&credential_path)?;
        let credential_path = &credential_path.join("starship-test-credential.json");
        let mut credential = File::create(credential_path)?;
        serde_json::to_writer_pretty(
            &mut credential,
            &serde_json::json!(
                {
                    "current": "https://api.example.com",
                    "accessTokens": {
                        "https://api.example.com": "redacted",
                        "https://api.pulumi.com": "redacted"
                    },
                    "accounts": {
                    }
                }
            ),
        )?;
        credential.sync_all()?;
        let rendered = ModuleRenderer::new("pulumi")
            .path(root.clone())
            .logical_path(root.clone())
            .config(toml::toml! {
                [pulumi]
                format = "via [$symbol($username@)$stack]($style) "
            })
            .env("HOME", root.to_str().unwrap())
            .collect();
        let expected = format!("via {} ", Color::Fixed(5).bold().paint(" launch"));
        assert_eq!(expected, rendered.expect("a result"));
        dir.close()?;
        Ok(())
    }

    #[test]
    fn empty_config_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let yaml = File::create(dir.path().join("Pulumi.yaml"))?;
        yaml.sync_all()?;

        let rendered = ModuleRenderer::new("pulumi")
            .path(dir.path())
            .logical_path(dir.path())
            .config(toml::toml! {
                [pulumi]
                format = "in [$symbol($stack)]($style) "
            })
            .collect();
        let expected = format!("in {} ", Color::Fixed(5).bold().paint(" "));
        assert_eq!(expected, rendered.expect("a result"));
        dir.close()?;
        Ok(())
    }

    #[test]
    fn do_not_search_upwards() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let child_dir = dir.path().join("child");
        std::fs::create_dir(&child_dir)?;
        let yaml = File::create(dir.path().join("Pulumi.yaml"))?;
        yaml.sync_all()?;

        let actual = ModuleRenderer::new("pulumi")
            .path(&child_dir)
            .logical_path(&child_dir)
            .config(toml::toml! {
                [pulumi]
                format = "in [$symbol($stack)]($style) "
                search_upwards = false
            })
            .collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn search_upwards_default() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let child_dir = dir.path().join("child");
        std::fs::create_dir(&child_dir)?;
        let yaml = File::create(dir.path().join("Pulumi.yaml"))?;
        yaml.sync_all()?;

        let actual = ModuleRenderer::new("pulumi")
            .path(&child_dir)
            .logical_path(&child_dir)
            .config(toml::toml! {
                [pulumi]
                format = "in [$symbol($stack)]($style) "
            })
            .collect();
        let expected = Some(format!("in {} ", Color::Fixed(5).bold().paint(" ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
