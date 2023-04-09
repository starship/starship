use super::{Context, Module, ModuleConfig};

use crate::configs::nodejs::NodejsConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

use once_cell::sync::Lazy;
use regex::Regex;
use semver::Version;
use semver::VersionReq;
use serde_json as json;
use std::ops::Deref;

/// Creates a module with the current Node.js version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("nodejs");
    let config = NodejsConfig::try_load(module.config);
    let is_js_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    let is_esy_project = context
        .try_begin_scan()?
        .set_folders(&["esy.lock"])
        .is_match();

    if !is_js_project || is_esy_project {
        return None;
    }

    let nodejs_version = Lazy::new(|| {
        context
            .exec_cmd("node", &["--version"])
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
                        check_engines_version(nodejs_version.as_deref(), engines_version);
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
                    let version = nodejs_version
                        .deref()
                        .as_ref()?
                        .trim_start_matches('v')
                        .trim();

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        version,
                        config.version_format,
                    )
                    .map(Ok)
                }
                _ => None,
            })
            .map(|variable| match variable {
                "expected_version" => {
                    let version = nodejs_version.as_deref();
                    let expected_version = get_engines_version(context);
                    let versions_in_range = check_engines_version(version, get_engines_version(context));

                    match (expected_version, versions_in_range) {
                        (Some(ver), false) => {
                            VersionFormatter::format_module_version(
                                module.get_name(),
                                &ver,
                                config.version_format,
                            )
                            .map(Ok)
                        }
                        _ => None,
                    }
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `nodejs`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_engines_version(context: &Context) -> Option<String> {
    let json_str = context.read_file_from_pwd("package.json")?;
    let package_json: json::Value = json::from_str(&json_str).ok()?;
    let raw_version = package_json.get("engines")?.get("node")?.as_str()?;
    Some(raw_version.to_string())
}

fn check_engines_version(nodejs_version: Option<&str>, engines_version: Option<String>) -> bool {
    let (Some(nodejs_version), Some(engines_version)) = (nodejs_version, engines_version) else {
        return true;
    };
    let Ok(r) = VersionReq::parse(&engines_version) else {
        return true;
    };
    let re = Regex::new(r"\d+\.\d+\.\d+").unwrap();
    let version = re
        .captures(nodejs_version)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str();
    let v = match Version::parse(version) {
        Ok(v) => v,
        Err(_e) => return true,
    };
    r.matches(&v)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::{self, File};
    use std::io;
    use std::io::Write;

    #[test]
    fn folder_without_node_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_package_json() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_package_json_and_esy_lock() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("package.json"))?.sync_all()?;
        let esy_lock = dir.path().join("esy.lock");
        fs::create_dir_all(esy_lock)?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_node_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".node-version"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_nvmrc() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".nvmrc"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_js_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.js"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mjs_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.mjs"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cjs_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.cjs"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_ts_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.ts"))?.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_node_modules() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let node_modules = dir.path().join("node_modules");
        fs::create_dir_all(node_modules)?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn engines_node_version_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"node\":\">=12.0.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn engines_node_version_not_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"node\":\"<12.0.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint(" v12.0.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn show_expected_version_when_engines_does_not_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"node\":\"11.0.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).config(toml::toml! {
            [nodejs]
            format = "via [$symbol($version )($expected_version )]($style)"
        }).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint(" v12.0.0 v11.0.0 ")));
    
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn do_not_show_expected_version_if_engines_match() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("package.json"))?;
        file.write_all(
            b"{
            \"engines\":{
                \"node\":\"12.0.0\"
            }
        }",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("nodejs").path(dir.path()).config(toml::toml! [
            [nodejs]
            format = "via [$symbol($version )($expected_version )]($style)"
        ]).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" v12.0.0 ")));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn no_node_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("index.js"))?.sync_all()?;
        let actual = ModuleRenderer::new("nodejs")
            .path(dir.path())
            .cmd("node --version", None)
            .collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint(" ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
