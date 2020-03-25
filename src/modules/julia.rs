use super::{Context, Module, RootModuleConfig};

use crate::configs::julia::JuliaConfig;
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

    let mut module = context.new_module("julia");
    let config: JuliaConfig = JuliaConfig::try_load(module.config);

    module.set_style(config.style);
    module.create_segment("symbol", &config.symbol);

    let formatted_version =
        format_julia_version(&utils::exec_cmd("julia", &["--version"])?.stdout.as_str())?;
    module.create_segment("version", &config.version.with_value(&formatted_version));

    Some(module)
}

fn format_julia_version(julia_stdout: &str) -> Option<String> {
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
    use tempfile;

    #[test]
    fn folder_without_julia_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = render_module("julia", dir.path());

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_julia_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.jl"))?.sync_all()?;

        let actual = render_module("julia", dir.path());

        let expected = Some(format!("via {} ", Color::Purple.bold().paint("👸 v1.4.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_project_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Project.toml"))?.sync_all()?;

        let actual = render_module("julia", dir.path());

        let expected = Some(format!("via {} ", Color::Purple.bold().paint("👸 v1.4.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_manifest_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Manifest.toml"))?.sync_all()?;

        let actual = render_module("julia", dir.path());

        let expected = Some(format!("via {} ", Color::Purple.bold().paint("👸 v1.4.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_format_julia_version() {
        let input = "julia version 1.4.0";
        assert_eq!(format_julia_version(input), Some("v1.4.0".to_string()));
    }
}
