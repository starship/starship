use super::{Context, Module, ModuleConfig};

use crate::configs::gleam::GleamConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Gleam version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("gleam");
    let config = GleamConfig::try_load(module.config);

    let is_gleam_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_gleam_project {
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
                    let gleam_version =
                        parse_gleam_version(&context.exec_cmd("gleam", &["--version"])?.stdout)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &gleam_version,
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
            log::warn!("Error in module `gleam`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_gleam_version(version: &str) -> Option<String> {
    let version = version.split_whitespace().last()?;
    Some(version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_folder_without_gleam_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("gleam").path(dir.path()).collect();

        let expected = None;

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_folder_with_gleam_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.gleam"))?.sync_all()?;

        let actual = ModuleRenderer::new("gleam").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Rgb(255, 175, 243).bold().paint("⭐ v1.0.0 ")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_folder_with_gleam_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("gleam.toml"))?.sync_all()?;

        let actual = ModuleRenderer::new("gleam").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Rgb(255, 175, 243).bold().paint("⭐ v1.0.0 ")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_parse_gleam_version() {
        let version = "gleam 1.0.0";
        assert_eq!(parse_gleam_version(version), Some("1.0.0".to_string()));
    }
}
