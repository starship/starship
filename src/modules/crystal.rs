use super::{Context, Module, RootModuleConfig};

use crate::configs::crystal::CrystalConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Crystal version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("crystal");
    let config: CrystalConfig = CrystalConfig::try_load(module.config);

    let is_crystal_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_crystal_project {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => format_crystal_version(
                    context.exec_cmd("crystal", &["--version"])?.stdout.as_str(),
                    config.version_format,
                ).map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `crystal`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn format_crystal_version(crystal_version: &str, version_format: &str) -> Option<String> {
    let version = crystal_version
        // split into ["Crystal", "0.35.1", ...]
        .split_whitespace()
        // return "0.35.1"
        .nth(1)?;

    match VersionFormatter::format_version(version, version_format) {
        Ok(formatted) => Some(formatted),
        Err(error) => {
            log::warn!("Error formating `crystal` version:\n{}", error);
            Some(format!("v{}", version))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::format_crystal_version;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_format_crystal_version() {
        assert_eq!(
            format_crystal_version("Crystal 0.35.1", "v${major}.${minor}.${patch}"),
            Some("v0.35.1".to_string())
        );
    }

    #[test]
    fn test_format_crystal_version_truncated() {
        assert_eq!(
            format_crystal_version("Crystal 0.35.1", "v${major}.${minor}"),
            Some("v0.35".to_string())
        );
    }

    #[test]
    fn test_format_crystal_version_is_malformed() {
        assert_eq!(
            format_crystal_version("Crystal 0.35", "v${major}.${minor}.${patch}"),
            Some("v0.35.".to_string())
        );
    }

    #[test]
    fn folder_without_crystal_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("crystal").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn folder_with_shard_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("shard.yml"))?.sync_all()?;

        let actual = ModuleRenderer::new("crystal").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint("🔮 v0.35.1 ")));
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn folder_with_cr_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.cr"))?.sync_all()?;

        let actual = ModuleRenderer::new("crystal").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint("🔮 v0.35.1 ")));
        assert_eq!(expected, actual);

        dir.close()
    }
}
