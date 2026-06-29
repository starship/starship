use super::{Context, Module, ModuleConfig};

use crate::configs::lean::LeanConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Gleam version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("lean");
    let config = LeanConfig::try_load(module.config);

    let is_lean_config = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_lean_config {
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
                    let lean_version =
                        parse_lean_version(&context.exec_cmd("lean", &["--version"])?.stdout)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &lean_version,
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
            log::warn!("Error in module `lean`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn parse_lean_version(version: &str) -> Option<String> {
    // Index of first literal "version " + jump to after space
    let idx_start_version = version.find("version ")? + 8; 
    let idx_after_version = idx_start_version + version[idx_start_version..]
                                    .find(|c : char| !(c.is_numeric() || c == '.'))?;

    
    let test = &version[idx_start_version..idx_after_version];
    Some(String::from(test))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_folder_without_lean_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("lean").path(dir.path()).collect();

        let expected = None;

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_folder_with_lean_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.lean"))?.sync_all()?;

        let actual = ModuleRenderer::new("lean").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Rgb(53, 145, 253).bold().paint("∀ 4.29.1 ")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_folder_with_lean_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("lakefile.toml"))?.sync_all()?;

        let actual = ModuleRenderer::new("lean").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Rgb(53, 145, 253).bold().paint("∀ 4.29.1 ")
        ));

        assert_eq!(actual, expected);
        dir.close()
    }

    #[test]
    fn test_parse_lean_version() {
        let version = "Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)";
        assert_eq!(parse_lean_version(version), Some("4.29.1".to_string()));
    }
}
