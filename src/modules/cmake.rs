use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::cmake::CMakeConfig;
use crate::utils;
use regex::Regex;

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

    let cmake_version = utils::exec_cmd("cmake", &["--version"])?.stdout;
    let formatted_version = Some(format!(
        "v{}",
        Regex::new(r"[0-9]+\.[0-9]+\.[0-9]+")
            .unwrap()
            .captures(&cmake_version)
            .unwrap()
            .get(0)
            .unwrap()
            .as_str()
    ))?;

    let mut module = context.new_module("cmake");
    let config: CMakeConfig = CMakeConfig::try_load(module.config);
    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(&formatted_version));

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;
    use tempfile;

    #[test]
    fn folder_without_cmake_lists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = render_module("cmake", dir.path());
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cmake_lists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("CMakeLists.txt"))?.sync_all()?;
        let actual = render_module("cmake", dir.path());
        let expected = Some(format!("via {} ", Color::Blue.bold().paint("🛆 v3.17.1")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
