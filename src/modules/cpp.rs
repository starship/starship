use super::{Context, Module, ModuleConfig};

use crate::configs::cpp::CppConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

use once_cell::sync::Lazy;
use semver::Version;
use std::borrow::Cow;
use std::ops::Deref;

/// Creates a module with the current C++ compiler and version
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
                        "clang++"
                    } else if cpp_compiler_info.contains("Free Software Foundation") {
                        "g++"
                    } else {
                        return None;
                    };
                    Some(cpp_compiler).map(Cow::Borrowed).map(Ok)
                }
                "version" => {
                    let cpp_compiler_info = &cpp_compiler_info.deref().as_ref()?.stdout;

                    // Clang says ...
                    //   Apple clang version 13.0.0 ...\n
                    //   OpenBSD clang version 11.1.0\n...
                    //   FreeBSD clang version 11.0.1 ...\n
                    // so we always want the first semver-ish whitespace-
                    // separated "word".
                    // c++/cpp/g++ says ...
                    //   g++ (OmniOS 151036/9.3.0-il-1) 9.3.0\n...
                    //   g++ (Debian 10.2.1-6) 10.2.1 ...\n
                    //   cpp (GCC) 3.3.5 (Debian 1:3.3.5-13)\n...
                    // so again we always want the first semver-ish word.
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        cpp_compiler_info.split_whitespace().find_map(
                            |word| match Version::parse(word) {
                                Ok(_v) => Some(word),
                                Err(_e) => None,
                            },
                        )?,
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
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_cpp_files() -> io::Result<()> {
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

        // What happens when `c++ --version` says it's modern g++?
        // The case when it claims to be clang++ is covered in folder_with_hpp_file,
        // and uses the mock in src/test/mod.rs.
        let actual = ModuleRenderer::new("cpp")
            .cmd(
                "c++ --version",
                Some(CommandOutput {
                    stdout: String::from(
                        "\
c++ (Debian 10.2.1-6) 10.2.1 20210110
Copyright (C) 2020 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.",
                    ),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Cyan.bold().paint("C++ v10.2.1-g++ ")
        ));
        assert_eq!(expected, actual);

        // What happens when `c++ --version` says it's ancient g++?
        let actual = ModuleRenderer::new("cpp")
            .cmd(
                "c++ --version",
                Some(CommandOutput {
                    stdout: String::from(
                        "\
c++ (GCC) 3.3.5 (Debian 1:3.3.5-13)
Copyright (C) 2003 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.",
                    ),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Cyan.bold().paint("C++ v3.3.5-g++ ")
        ));
        assert_eq!(expected, actual);

        // What happens when `cpp --version` says it's modern g++?
        // The case when it claims to be clang++ is covered in folder_with_hpp_file,
        // and uses the mock in src/test/mod.rs.
        let actual = ModuleRenderer::new("cpp")
            .cmd(
                "cpp --version",
                Some(CommandOutput {
                    stdout: String::from(
                        "\
cpp (Debian 10.2.1-6) 10.2.1 20210110
Copyright (C) 2020 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.",
                    ),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Cyan.bold().paint("C++ v10.2.1-g++ ")
        ));
        assert_eq!(expected, actual);

        // What happens when `cpp --version` says it's ancient g++?
        let actual = ModuleRenderer::new("cpp")
            .cmd(
                "cpp --version",
                Some(CommandOutput {
                    stdout: String::from(
                        "\
cpp (GCC) 3.3.5 (Debian 1:3.3.5-13)
Copyright (C) 2003 Free Software Foundation, Inc.
This is free software; see the source for copying conditions.  There is NO
warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.",
                    ),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Cyan.bold().paint("C++ v3.3.5-g++ ")
        ));
        assert_eq!(expected, actual);

        // What happens with an unknown C++ compiler? Needless to say, we're
        // not running on a Z80 so we're never going to see this one in reality!
        let actual = ModuleRenderer::new("cpp")
            .cmd(
                "cpp --version",
                Some(CommandOutput {
                    stdout: String::from("HISOFT-C++ Compiler  V1.2\nCopyright © 1984 HISOFT"),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint("C++ ")));
        assert_eq!(expected, actual);

        // What happens when 'c++ --version' doesn't work, but 'g++ --version' does?
        // This stubs out `c++` but we'll fall back to `g++ --version` as defined in
        // src/test/mod.rs.
        // Also we don't bother to redefine the config for this one, as we've already
        // proved we can parse its version.
        let actual = ModuleRenderer::new("cpp")
            .cmd("c++ --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Cyan.bold().paint("C++ v10.2.1-g++ ")
        ));
        assert_eq!(expected, actual);

        // What happens when 'cpp --version' doesn't work, but 'g++ --version' does?
        // This stubs out `cpp` but we'll fall back to `g++ --version` as defined in
        // src/test/mod.rs.
        // Also we don't bother to redefine the config for this one, as we've already
        // proved we can parse its version.
        let actual = ModuleRenderer::new("cpp")
            .cmd("cpp --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Cyan.bold().paint("C++ v10.2.1-g++ ")
        ));
        assert_eq!(expected, actual);

        // Now with 'c++', 'cpp' and 'g++' not working, this should fall back to 'clang++ --version'
        let actual = ModuleRenderer::new("cpp")
            .cmd("c++ --version", None)
            .cmd("cpp --version", None)
            .cmd("g++ --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Cyan.bold().paint("C++ v11.1.0-clang++ ")
        ));
        assert_eq!(expected, actual);

        // What happens when we can't find any of c++, cpp, g++ or clang++?
        let actual = ModuleRenderer::new("cpp")
            .cmd("c++ --version", None)
            .cmd("cpp --version", None)
            .cmd("g++ --version", None)
            .cmd("clang++ --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint("C++ ")));
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
            Color::Cyan.bold().paint("C++ v11.0.1-clang++ ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
