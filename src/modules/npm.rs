use super::{Context, Module, ModuleConfig};

use crate::configs::npm::NpmConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

use once_cell::sync::Lazy;
use regex::Regex;
use semver::Version;
use semver::VersionReq;
use serde_json as json;
use std::ops::Deref;

/// Creates a module with the current Node.js version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("npm");
    let config = NpmConfig::try_load(module.config);
    let is_npm_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .is_match();

    if !is_npm_project {
        return None;
    }

    let npm_version = Lazy::new(|| {
        context
            .exec_cmd("npm", &["--version"])
            .map(|cmd| cmd.stdout)
    });
    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => {
                    let engines_version = get_engines_version(context);
                    let in_engines_range =
                        check_engines_version(npm_version.deref().as_ref()?, engines_version);
                    if in_engines_range {
                        Some(Ok(config.style))
                    } else {
                        Some(Ok(config.not_capable_style))
                    }
                }
                _ => None,
            })
            .map(|variable| match variable {
                "version" => {
                    let version = npm_version.deref().as_ref()?.trim_start_matches('v').trim();

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        version,
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
            log::warn!("Error in module `npm`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_engines_version(context: &Context) -> Option<String> {
    let json_str = context.read_file_from_pwd("package.json")?;
    let package_json: json::Value = json::from_str(&json_str).ok()?;
    let raw_version = package_json.get("engines")?.get("npm")?.as_str()?;
    Some(raw_version.to_string())
}

fn check_engines_version(npm_version: &str, engines_version: Option<String>) -> bool {
    if engines_version.is_none() {
        return true;
    }
    let r = match VersionReq::parse(&engines_version.unwrap()) {
        Ok(r) => r,
        Err(_e) => return true,
    };
    let re = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    let version = re.captures(npm_version).unwrap().get(0).unwrap().as_str();
    let v = match Version::parse(version) {
        Ok(v) => v,
        Err(_e) => return true,
    };
    r.matches(&v)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;
    use std::io::Write;

    #[test]
    fn config_blank() {
        let actual = ModuleRenderer::new("npm").collect();

        let expected = None;
        assert_eq!(expected, actual);
    }

    #[test]
    fn folder_without_npm_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("npm")
            .config(toml::toml! {
                [npm]
                disabled = false
            })
            .path(dir.path())
            .collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_package_lock() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package-lock.json"))?.sync_all()?;

        let actual = ModuleRenderer::new("npm")
            .config(toml::toml! {
                [npm]
                disabled = false
            })
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(88).bold().paint(" v8.1.0 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_npmrc() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".npmrc"))?.sync_all()?;

        let actual = ModuleRenderer::new("npm")
            .config(toml::toml! {
                [npm]
                disabled = false
            })
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(88).bold().paint(" v8.1.0 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn engines_npm_version_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package-lock.json"))?.sync_all()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"npm\":\">=8.1.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("npm")
            .config(toml::toml! {
                [npm]
                disabled = false
            })
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(88).bold().paint(" v8.1.0 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn engines_npm_version_not_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package-lock.json"))?.sync_all()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"npm\":\"<8.1.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("npm")
            .config(toml::toml! {
                [npm]
                disabled = false
            })
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint(" v8.1.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
