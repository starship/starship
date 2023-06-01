use super::{Context, Module, ModuleConfig};

use crate::configs::go::GoConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

use once_cell::sync::Lazy;
use regex::Regex;
use semver::Version;
use semver::VersionReq;
use std::ops::Deref;

/// Creates a module with the current Go version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("golang");
    let config = GoConfig::try_load(module.config);
    let is_go_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_go_project {
        return None;
    }

    let golang_version =
        Lazy::new(|| parse_go_version(&context.exec_cmd("go", &["version"])?.stdout));
    let mod_version = Lazy::new(|| get_go_mod_version(context));

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => {
                    let in_mod_range =
                        check_go_version(golang_version.as_deref(), mod_version.as_deref());

                    if in_mod_range {
                        Some(Ok(config.style))
                    } else {
                        Some(Ok(config.not_capable_style))
                    }
                }
                _ => None,
            })
            .map(|variable| match variable {
                "version" => {
                    let go_ver = golang_version.deref().as_ref()?;

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        go_ver,
                        config.version_format,
                    )
                    .map(Ok)
                }
                "mod_version" => {
                    let in_mod_range =
                        check_go_version(golang_version.as_deref(), mod_version.as_deref());
                    let mod_ver = mod_version.as_deref()?.to_string();

                    (!in_mod_range).then_some(Ok(mod_ver))
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `golang`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_go_version(go_stdout: &str) -> Option<String> {
    // go version output looks like this:
    // go version go1.13.3 linux/amd64

    let version = go_stdout
        // split into ("", "1.12.4 linux/amd64")
        .split_once("go version go")?
        // return "1.12.4 linux/amd64"
        .1
        // split into ["1.12.4", "linux/amd64"]
        .split_whitespace()
        // return "1.12.4"
        .next()?;

    Some(version.to_string())
}

fn get_go_mod_version(context: &Context) -> Option<String> {
    let mod_str = context.read_file_from_pwd("go.mod")?;
    let re = Regex::new(r"(?:go\s)(\d+(\.\d+)+)").unwrap();

    if let Some(cap) = re.captures(&mod_str) {
        let mod_ver = cap.get(1)?.as_str();
        Some(mod_ver.to_string())
    } else {
        None
    }
}

fn check_go_version(go_version: Option<&str>, mod_version: Option<&str>) -> bool {
    let (Some(go_version), Some(mod_version)) = (go_version, mod_version) else {
        return true;
    };
    let Ok(r) = VersionReq::parse(mod_version) else {
        return true;
    };
    let Ok(v) = Version::parse(go_version) else {
        return true;
    };

    r.matches(&v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::{self, File};
    use std::io;
    use std::io::Write;

    #[test]
    fn folder_without_go_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("golang").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_go_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.go"))?.sync_all()?;

        let actual = ModuleRenderer::new("golang").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🐹 v1.12.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_go_mod() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("go.mod"))?.sync_all()?;

        let actual = ModuleRenderer::new("golang").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🐹 v1.12.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_go_sum() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("go.sum"))?.sync_all()?;

        let actual = ModuleRenderer::new("golang").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🐹 v1.12.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_go_work() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("go.work"))?.sync_all()?;

        let actual = ModuleRenderer::new("golang").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🐹 v1.12.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_godeps() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let godeps = dir.path().join("Godeps");
        fs::create_dir_all(godeps)?;

        let actual = ModuleRenderer::new("golang").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🐹 v1.12.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_glide_yaml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("glide.yaml"))?.sync_all()?;

        let actual = ModuleRenderer::new("golang").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🐹 v1.12.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_gopkg_yml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Gopkg.yml"))?.sync_all()?;

        let actual = ModuleRenderer::new("golang").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🐹 v1.12.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
    #[test]
    fn folder_with_gopkg_lock() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Gopkg.lock"))?.sync_all()?;

        let actual = ModuleRenderer::new("golang").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🐹 v1.12.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
    #[test]
    fn folder_with_go_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".go-version"))?.sync_all()?;

        let actual = ModuleRenderer::new("golang").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🐹 v1.12.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_format_go_version() {
        let input = "go version go1.12 darwin/amd64";
        assert_eq!(parse_go_version(input), Some("1.12".to_string()));
    }

    #[test]
    fn show_mod_version_if_not_matching_go_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("go.mod"))?;
        file.write_all(
            b"package test\n\n
            go 1.16",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("golang")
            .path(dir.path())
            .config(toml::toml! {
                [golang]
                format = "via [$symbol($version )($mod_version )]($style)"
            })
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Red.bold().paint("🐹 v1.12.1 1.16 ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn hide_mod_version_when_it_matches_go_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("go.mod"))?;
        file.write_all(
            b"package test\n\n
            go 1.12",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("golang")
            .path(dir.path())
            .config(toml::toml! {
                [golang]
                format = "via [$symbol($version )($mod_version )]($style)"
            })
            .collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🐹 v1.12.1 ")));

        assert_eq!(expected, actual);
        dir.close()
    }
}
