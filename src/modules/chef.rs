use super::{Context, Module, RootModuleConfig};

use crate::configs::chef::ChefConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;
use crate::utils;

use regex::Regex;
use std::fs;
use std::path::Path;

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
                    let chef_version = version_from_chef_gem()?;

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &chef_version,
                        config.version_format,
                    )
                    .map(Ok)
                }
                _ => None,
            })
            .parse(None, Some(context))
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

fn version_from_chef_gem() -> Option<String> {
    // Find the embedded Ruby used by the chef-client
    let chef_client_path = find_chef_client()?;
    let cc_str = chef_client_path.as_str();
    let chef_client_contents = utils::read_file(cc_str.to_string()).ok()?;

    // Get Ruby's path
    let re = Regex::new(r#"#!(.+)/ruby --disable-gems"#).unwrap();
    let ruby_path = re
        .captures(chef_client_contents.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    // /opt/chefdk/embedded is slightly irrelevant as it can be installed elsewhere
    // /opt/chefdk/embedded/bin
    let path = Path::new(ruby_path);
    // /opt/chefdk/embedded
    let base = path.parent().unwrap();
    // /opt/chefdk/embedded/lib/ruby/gems
    let gems_path = base.join("lib").join("ruby").join("gems");

    // 2.5.0, chef only packs one version of ruby and therefor one set of gems
    let ruby_version = match fs::read_dir(gems_path).unwrap().nth(0).unwrap() {
        Err(error) => {
            log::warn!("Error in module `chef`:\n{}", error);
            return None;
        }
        Ok(version) => {
            version
        },
    };
    let ruby_version_string = ruby_version.path().into_os_string().into_string().unwrap();

    // /opt/chefdk/embedded/lib/ruby/gems/2.5.0/gems
    let full_gems_path = base
        .join("lib")
        .join("ruby")
        .join("gems")
        .join(ruby_version_string)
        .join("gems");

    // gem format chef-x.x.x
    let re = Regex::new(r"chef-\d+\.\d+\.\d+").unwrap();

    if full_gems_path.is_dir() {
        // Iterate over directories
        for entry in fs::read_dir(full_gems_path).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();

            if let Some(file_name) = path.file_name().unwrap().to_str() {
                // Match the chef gem
                if re.is_match(file_name) {
                    // Split the version off
                    let v = file_name.split("chef-");

                    let version: Vec<&str> = v.collect();
                    return Some(version[1].to_string())
                }
            }
        }
    }

    None
}

fn find_chef_client() -> Option<String> {
    // Get the chef-client path
    let chef_client_path = which::which("chef-client");
    let full_path = match chef_client_path {
        Ok(full_path) => {
            log::trace!("Using {:?} as chef-client", full_path);
            full_path
        }
        Err(error) => {
            log::warn!("Error in module `chef`:\n{}", error);
            log::warn!("Unable to find chef-client in PATH, {:?}", error);
            return None;
        }
    };

    let path_str = full_path.into_os_string().into_string().unwrap();
    Some(path_str)
}

#[cfg(test)]
mod tests {
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
    fn folder_with_policyfile_rb() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Policyfile.rb"))?.sync_all()?;

        let actual = ModuleRenderer::new("chef").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Red.bold().paint("🍳 v12.21.14 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    // #[test]
    // fn test_format_chef_version() {
    //     let input = "Chef: 12.21.14";
    //     assert_eq!(parse_chef_version(input), Some("12.21.14".to_string()));
    // }
}
