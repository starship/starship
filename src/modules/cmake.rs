use super::{Context, Module, RootModuleConfig};

use crate::configs::cmake::CMakeConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current CMake version
///
/// Will display the CMake version if any of the following criteria are met:
///     - The current directory contains a `CMakeLists.txt` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_cmake_project = context
        .try_begin_scan()?
        .set_files(&["CMakeLists.txt"])
        .is_match();

    if !is_cmake_project {
        return None;
    }

    let mut module = context.new_module("cmake");
    let config = CMakeConfig::try_load(module.config);
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
                "version" => utils::exec_cmd("cmake", &["--version"])
                    .map(|output| format_cmake_version(&output.stdout))
                    .flatten()
                    .map(Ok),
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

fn format_cmake_version(cmake_version: &str) -> Option<String> {
    let version = cmake_version.split_whitespace().nth(2)?;
    Some(format!("v{}", version))
}

#[cfg(test)]
mod tests {
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_cmake_lists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("cmake", dir.path(), None);
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cmake_lists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("CMakeLists.txt"))?.sync_all()?;
        let actual = render_module("cmake", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Blue.bold().paint("🛆 v3.17.3")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
