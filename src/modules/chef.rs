use super::{Context, Module, RootModuleConfig};

use crate::configs::chef::ChefConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Chef Client version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("chef");
    let config = ChefConfig::try_load(module.config);
    let is_chef_cookbook = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_chef_cookbook {
        return None;
    }

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
                    let chef_version =
                        parse_chef_version(&context.exec_cmd("chef-client", &["--version"])?.stdout)?;

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &chef_version,
                        config.version_format,
                    )
                    .map(Ok)
                }
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `chef`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_chef_version(chef_stdout: &str) -> Option<String> {
    let version = chef_stdout
        .splitn(2, "Chef: ")
        .nth(1)?;

    Some(version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::{self, File};
    use std::io;

    #[test]
    fn folder_without_chef_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("chef").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_metadata_rb() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("metadata.rb"))?.sync_all()?;

        let actual = ModuleRenderer::new("chef").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Red.bold().paint("🍳 v12.21.14 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_metadata_json() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("metadata.json"))?.sync_all()?;

        let actual = ModuleRenderer::new("chef").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Red.bold().paint("🍳 v12.21.14 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cookbooks() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let cookbooks = dir.path().join("cookbooks");
        fs::create_dir_all(&cookbooks)?;

        let actual = ModuleRenderer::new("chef").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Red.bold().paint("🍳 v12.21.14 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_recipes() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let recipes = dir.path().join("recipes");
        fs::create_dir_all(&recipes)?;

        let actual = ModuleRenderer::new("chef").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Red.bold().paint("🍳 v12.21.14 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_berksfile() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Berksfile"))?.sync_all()?;

        let actual = ModuleRenderer::new("chef").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Red.bold().paint("🍳 v12.21.14 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_berksfile_lock() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Berksfile.lock"))?.sync_all()?;

        let actual = ModuleRenderer::new("chef").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Red.bold().paint("🍳 v12.21.14 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dot_kitchen_yml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".kitchen.yml"))?.sync_all()?;

        let actual = ModuleRenderer::new("chef").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Red.bold().paint("🍳 v12.21.14 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_kitchen_yml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("kitchen.yml"))?.sync_all()?;

        let actual = ModuleRenderer::new("chef").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Red.bold().paint("🍳 v12.21.14 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_format_chef_version() {
        let input = "Chef: 12.21.14";
        assert_eq!(parse_chef_version(input), Some("12.21.14".to_string()));
    }
}
