use super::{Context, Module, ModuleConfig};

use crate::configs::go::GoConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

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
                    let golang_version =
                        parse_go_version(&context.exec_cmd("go", &["version"])?.stdout)?;

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &golang_version,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::{self, File};
    use std::io;

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
        fs::create_dir_all(&godeps)?;

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
}
