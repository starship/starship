use super::{Context, Module, RootModuleConfig};

use crate::configs::go::GoConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current Go version
///
/// Will display the Go version if any of the following criteria are met:
///     - Current directory contains a `go.mod` file
///     - Current directory contains a `go.sum` file
///     - Current directory contains a `glide.yaml` file
///     - Current directory contains a `Gopkg.yml` file
///     - Current directory contains a `Gopkg.lock` file
///     - Current directory contains a `.go-version` file
///     - Current directory contains a `Godeps` directory
///     - Current directory contains a file with the `.go` extension
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_go_project = context
        .try_begin_scan()?
        .set_files(&[
            "go.mod",
            "go.sum",
            "glide.yaml",
            "Gopkg.yml",
            "Gopkg.lock",
            ".go-version",
        ])
        .set_extensions(&["go"])
        .set_folders(&["Godeps"])
        .is_match();

    if !is_go_project {
        return None;
    }

    let mut module = context.new_module("golang");
    let config = GoConfig::try_load(module.config);
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
                    format_go_version(&utils::exec_cmd("go", &["version"])?.stdout.as_str()).map(Ok)
                }
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `golang`:\n{}", error);
            return None;
        }
    });

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    Some(module)
}

fn format_go_version(go_stdout: &str) -> Option<String> {
    // go version output looks like this:
    // go version go1.13.3 linux/amd64

    let version = go_stdout
        // split into ["", "1.12.4 linux/amd64"]
        .splitn(2, "go version go")
        // return "1.12.4 linux/amd64"
        .nth(1)?
        // split into ["1.12.4", "linux/amd64"]
        .split_whitespace()
        // return "1.12.4"
        .next()?;

    Some(format!("v{}", version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::{self, File};
    use std::io;

    #[test]
    fn folder_without_go_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = render_module("golang", dir.path(), None);

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_go_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.go"))?.sync_all()?;

        let actual = render_module("golang", dir.path(), None);

        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_go_mod() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("go.mod"))?.sync_all()?;

        let actual = render_module("golang", dir.path(), None);

        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_go_sum() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("go.sum"))?.sync_all()?;

        let actual = render_module("golang", dir.path(), None);

        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_godeps() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let godeps = dir.path().join("Godeps");
        fs::create_dir_all(&godeps)?;

        let actual = render_module("golang", dir.path(), None);

        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_glide_yaml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("glide.yaml"))?.sync_all()?;

        let actual = render_module("golang", dir.path(), None);

        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_gopkg_yml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Gopkg.yml"))?.sync_all()?;

        let actual = render_module("golang", dir.path(), None);

        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1")));
        assert_eq!(expected, actual);
        dir.close()
    }
    #[test]
    fn folder_with_gopkg_lock() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Gopkg.lock"))?.sync_all()?;

        let actual = render_module("golang", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1")));
        assert_eq!(expected, actual);
        dir.close()
    }
    #[test]
    fn folder_with_go_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".go-version"))?.sync_all()?;

        let actual = render_module("golang", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Cyan.bold().paint("🐹 v1.12.1")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_format_go_version() {
        let input = "go version go1.12 darwin/amd64";
        assert_eq!(format_go_version(input), Some("v1.12".to_string()));
    }
}
