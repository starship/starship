use super::{Context, Module, RootModuleConfig};
use crate::formatter::VersionFormatter;

use crate::configs::cmake::CMakeConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current CMake version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("cmake");
    let config = CMakeConfig::try_load(module.config);

    let is_cmake_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_cmake_project {
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
                "version" => {
                    let cmake_version = context.exec_cmd("cmake", &["--version"])?.stdout;
                    format_cmake_version(&cmake_version, config.version_format).map(Ok)
                }
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `cmake`: \n{}", error);
            return None;
        }
    });

    Some(module)
}

fn format_cmake_version(cmake_version: &str, version_format: &str) -> Option<String> {
    let version = cmake_version
        //split into ["cmake" "version" "3.10.2", ...]
        .split_whitespace()
        // get down to "3.10.2"
        .nth(2)?;

    match VersionFormatter::format_version(version, version_format) {
        Ok(formatted) => Some(formatted),
        Err(error) => {
            log::warn!("Error formatting `cmake` version:\n{}", error);
            Some(format!("v{}", version))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::format_cmake_version;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_format_cmake_version() {
        assert_eq!(
            format_cmake_version("cmake version 3.10.2", "v${major}.${minor}.${patch}"),
            Some("v3.10.2".to_string())
        );
    }

    #[test]
    fn test_format_cmake_version_truncated() {
        assert_eq!(
            format_cmake_version("cmake version 3.10.2", "v${major}.${minor}"),
            Some("v3.10".to_string())
        );
    }

    #[test]
    fn test_format_cmake_version_is_malformed() {
        assert_eq!(
            format_cmake_version("cmake version 3.10", "v${major}.${minor}.${patch}"),
            Some("v3.10.".to_string())
        );
    }

    #[test]
    fn folder_without_cmake_lists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("cmake").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cmake_lists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("CMakeLists.txt"))?.sync_all()?;
        let actual = ModuleRenderer::new("cmake").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("△ v3.17.3 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn buildfolder_with_cmake_cache() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("CMakeCache.txt"))?.sync_all()?;
        let actual = ModuleRenderer::new("cmake").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("△ v3.17.3 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
