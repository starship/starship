use super::{Context, Module, ModuleConfig};

use crate::configs::godot::GodotConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

use regex::Regex;

/// Creates a module with the current Godot version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("godot");
    let config = GodotConfig::try_load(module.config);

    let is_gd_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_gd_project {
        return None;
    }

    let raw_godot_version = get_godot_version(config.godot_command, context)?;

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => {
                    format_godot_version(raw_godot_version.as_str(), config.version_format).map(Ok)
                }
                "numver" => {
                    format_godot_version(raw_godot_version.as_str(), "${major}.${minor}.${patch}")
                        .map(Ok)
                }
                "fullver" => Some(Ok(raw_godot_version.clone())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `rust`:\n{error}");
            return None;
        }
    });

    Some(module)
}

/// Get the output for `godot --version`
fn get_godot_version(command: &str, context: &Context) -> Option<String> {
    context
        .exec_cmd(command, &["--version"])?
        .stdout
        .split_whitespace()
        .next()
        .map(str::to_owned)
}

fn format_godot_version(version: &str, version_format: &str) -> Option<String> {
    let re = Regex::new(r"(^\d+\.\d+\.\d+)").unwrap();
    let ver = match re.captures(version) {
        Some(caps) => caps[0].to_string(),
        _ => version.to_string(),
    };
    match VersionFormatter::format_version(ver.as_str(), version_format) {
        Ok(formatted) => Some(formatted),
        Err(error) => {
            log::warn!("Error formatting `godot` version:\n{error}");
            Some(format!("v{version}"))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::{self, File};
    use std::io;

    #[test]
    fn test_format_godot_version() {
        let config = GodotConfig::default();
        let godot_linux = "4.6.1.stable.arch_linux.14d19694e";
        let godot_windows = "4.6.1.stable.official.14d19694e";

        assert_eq!(
            format_godot_version(godot_linux, config.version_format),
            Some("v4.6.1".to_string())
        );

        assert_eq!(
            format_godot_version(godot_windows, config.version_format),
            Some("v4.6.1".to_string())
        );

        assert_eq!(
            format_godot_version(
                godot_linux,
                "major:${major} minor:${minor} patch:${patch} raw:${raw}"
            ),
            Some("major:4 minor:6 patch:1 raw:4.6.1".to_string())
        );
    }

    #[test]
    fn folder_without_godot_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("godot").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_godot_gdscript_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("custom_node.gd"))?.sync_all()?;

        let actual = ModuleRenderer::new("godot").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Blue.bold().paint(" v4.6.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_godot_project_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("project.godot"))?.sync_all()?;

        let actual = ModuleRenderer::new("godot").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Blue.bold().paint(" v4.6.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_godot_resource_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("big_bad_boss.tres"))?.sync_all()?;

        let actual = ModuleRenderer::new("godot").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Blue.bold().paint(" v4.6.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_godot_scene_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("level1.tscn"))?.sync_all()?;

        let actual = ModuleRenderer::new("godot").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Blue.bold().paint(" v4.6.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_godot_metadata_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let godeps = dir.path().join(".godot");
        fs::create_dir_all(godeps)?;

        let actual = ModuleRenderer::new("godot").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Blue.bold().paint(" v4.6.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
