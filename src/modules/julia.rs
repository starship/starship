use super::{Context, Module, RootModuleConfig};

use crate::configs::julia::JuliaConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current Julia version
///
/// Will display the Julia version if any of the following criteria are met:
///     - Current directory contains a `Project.toml` file
///     - Current directory contains a `Manifest.toml` file
///     - Current directory contains a file with the `.jl` extension
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_julia_project = context
        .try_begin_scan()?
        .set_files(&["Project.toml", "Manifest.toml"])
        .set_extensions(&["jl"])
        .is_match();

    if !is_julia_project {
        return None;
    }

    let julia_version = utils::exec_cmd("julia", &["--version"])?.stdout;

    let mut module = context.new_module("julia");
    let config = JuliaConfig::try_load(module.config);
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
                "version" => parse_julia_version(&julia_version).map(Ok),
                _ => None,
            })
            .parse(None)
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
        // split into ["", "1.4.0"]
        .splitn(2, "julia version")
        // return "1.4.0"
        .nth(1)?
        .split_whitespace()
        .next()?;

    Some(format!("v{}", version))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_julia_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = render_module("julia", dir.path(), None);

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_julia_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.jl"))?.sync_all()?;

        let actual = render_module("julia", dir.path(), None);

        let expected = Some(format!("via {} ", Color::Purple.bold().paint("ஃ v1.4.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_project_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Project.toml"))?.sync_all()?;

        let actual = render_module("julia", dir.path(), None);

        let expected = Some(format!("via {} ", Color::Purple.bold().paint("ஃ v1.4.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_manifest_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Manifest.toml"))?.sync_all()?;

        let actual = render_module("julia", dir.path(), None);

        let expected = Some(format!("via {} ", Color::Purple.bold().paint("ஃ v1.4.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_parse_julia_version() {
        const OUTPUT: &str = "julia version 1.4.0";
        assert_eq!(parse_julia_version(OUTPUT), Some("v1.4.0".to_string()));
    }
}
