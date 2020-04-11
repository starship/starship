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

    let cmake_version = utils::exec_cmd("cmake", &["--version"])?.stdout;
    let module_version = format_cmake_version(&cmake_version)?;

    let mut module = context.new_module("cmake");
    let config: CMakeConfig = CMakeConfig::try_load(module.config);
    let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
        formatter.map(|variable| match variable {
            "version" => Some(module_version.clone()),
            _ => None,
        })
    } else {
        log::warn!("Error parsing format string in `cmake.format`");
        return None;
    };

    module.set_segments(formatter.parse(None));

    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    Some(module)
}

fn format_cmake_version(cmake_version: &str) -> Option<String> {
    let version = cmake_version.split_whitespace().nth(2)?;
    let mut formatted_version = String::with_capacity(version.len() + 1);
    formatted_version.push('v');
    formatted_version.push_str(version);
    Some(formatted_version)
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
