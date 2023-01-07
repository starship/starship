use super::{Context, Module, ModuleConfig};

use crate::configs::julia::JuliaConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Julia version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("julia");
    let config = JuliaConfig::try_load(module.config);

    let is_julia_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_julia_project {
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
                    let julia_version =
                        parse_julia_version(&context.exec_cmd("julia", &["--version"])?.stdout)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &julia_version,
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
            log::warn!("Error in module `julia`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_julia_version(julia_stdout: &str) -> Option<String> {
    // julia version output looks like this:
    // julia version 1.4.0

    let version = julia_stdout
        // split into ("", "1.4.0")
        .split_once("julia version")?
        // return "1.4.0"
        .1
        .split_whitespace()
        .next()?;

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
    fn folder_without_julia_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("julia").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_julia_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.jl"))?.sync_all()?;

        let actual = ModuleRenderer::new("julia").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Purple.bold().paint("ஃ v1.4.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_project_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Project.toml"))?.sync_all()?;

        let actual = ModuleRenderer::new("julia").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Purple.bold().paint("ஃ v1.4.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_manifest_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Manifest.toml"))?.sync_all()?;

        let actual = ModuleRenderer::new("julia").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Purple.bold().paint("ஃ v1.4.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_parse_julia_version() {
        let input = "julia version 1.4.0";
        assert_eq!(parse_julia_version(input), Some("1.4.0".to_string()));
    }
}
