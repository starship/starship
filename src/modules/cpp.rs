use super::{Context, Module, ModuleConfig};

use crate::configs::cpp::CppConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

use once_cell::sync::Lazy;
use semver::Version;
use std::borrow::Cow;
use std::ops::Deref;

/// Creates a module with the current C compiler and version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("cpp");
    let config: CppConfig = CppConfig::try_load(module.config);
    let is_cpp_project = context
        .try_begin_scan()?
        .set_extensions(&config.detect_extensions)
        .set_files(&config.detect_files)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_cpp_project {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        let cpp_compiler_info = Lazy::new(|| context.exec_cmds_return_first(config.commands));

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
                "name" => {
                    let cpp_compiler_info = &cpp_compiler_info.deref().as_ref()?.stdout;

                    let cpp_compiler = if cpp_compiler_info.contains("clang") {
                        "clang"
                    } else if cpp_compiler_info.contains("Free Software Foundation") {
                        "g++"
                    } else {
                        return None;
                    };
                    Some(cpp_compiler).map(Cow::Borrowed).map(Ok)
                }
                "version" => {
                    let compiler_info = &cpp_compiler_info.deref().as_ref()?.stdout;

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        compiler_info
                            .split_whitespace()
                            .find(|word| Version::parse(word).is_ok())?,
                        config.version_format,
                    )
                    .map(Cow::Owned)
                    .map(Ok)
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `cpp`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_c_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("cpp").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cpp_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.cpp"))?.sync_all()?;

        let actual = ModuleRenderer::new("cpp").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C++ v11.3.0-g++ ")
        ));
        assert_eq!(expected, actual);

        //If g++ does not exist, it should get clang
        let actual = ModuleRenderer::new("cpp")
            .cmd("g++ --version", None)
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C++ v11.1.0-clang ")
        ));

        assert_eq!(expected, actual);

        //If either does not exist print just C++
        let actual = ModuleRenderer::new("cpp")
            .cmd("g++ --version", None)
            .cmd("clang --version", None)
            .path(dir.path())
            .collect();

        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C++ ")));

        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn folder_with_hpp_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.hpp"))?.sync_all()?;

        let actual = ModuleRenderer::new("cpp").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C++ v11.3.0-g++ ")
        ));
        assert_eq!(expected, actual);

        //If g++ does not exist, it should get clang
        let actual = ModuleRenderer::new("cpp")
            .cmd("g++ --version", None)
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C++ v11.1.0-clang ")
        ));

        assert_eq!(expected, actual);

        //If either does not exist print just C++
        let actual = ModuleRenderer::new("cpp")
            .cmd("g++ --version", None)
            .cmd("clang --version", None)
            .path(dir.path())
            .collect();

        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C++ ")));

        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn folder_with_cc_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.cc"))?.sync_all()?;

        let actual = ModuleRenderer::new("cpp").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C++ v11.3.0-g++ ")
        ));
        assert_eq!(expected, actual);

        //If g++ does not exist, it should get clang
        let actual = ModuleRenderer::new("cpp")
            .cmd("g++ --version", None)
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C++ v11.1.0-clang ")
        ));

        assert_eq!(expected, actual);

        //If either does not exist print just C++
        let actual = ModuleRenderer::new("cpp")
            .cmd("g++ --version", None)
            .cmd("clang --version", None)
            .path(dir.path())
            .collect();

        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C++ ")));

        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn folder_with_cxx_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.cxx"))?.sync_all()?;

        let actual = ModuleRenderer::new("cpp").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C++ v11.3.0-g++ ")
        ));
        assert_eq!(expected, actual);

        //If g++ does not exist, it should get clang
        let actual = ModuleRenderer::new("cpp")
            .cmd("g++ --version", None)
            .path(dir.path())
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C++ v11.1.0-clang ")
        ));

        assert_eq!(expected, actual);

        //If either does not exist print just C++
        let actual = ModuleRenderer::new("cpp")
            .cmd("g++ --version", None)
            .cmd("clang --version", None)
            .path(dir.path())
            .collect();

        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C++ ")));

        assert_eq!(expected, actual);

        dir.close()
    }
}
