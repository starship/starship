use super::{Context, Module};
use crate::modules::cc::{Lang, module as cc_module};

/// Creates a module with the current C compiler and version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    cc_module(context, Lang::Cpp)
}

#[cfg(test)]
mod tests {
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_cpp_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("cpp")
            .config(toml::toml! {
                [cpp]
                    disabled = false
            })
            .path(dir.path())
            .collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cpp_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.cpp"))?.sync_all()?;

        // What happens when `c++ --version` says it's modern clang++?
        // The case when it claims to be g++ is covered in folder_with_hpp_file,
        let actual = ModuleRenderer::new("cpp")
            .config(toml::toml! {
                [cpp]
                    disabled = false
            })
            .cmd(
                "c++ --version",
                Some(CommandOutput {
                    stdout: String::from(
                        "\
clang version 19.1.7
Target: x86_64-pc-linux-gnu
Thread model: posix
InstalledDir: /usr/bin",
                    ),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C++ v19.1.7-clang++ ")
        ));
        assert_eq!(expected, actual);

        // What happens when `c++ --version` says it's ancient gcc?
        let actual = ModuleRenderer::new("cpp")
            .config(toml::toml! {
                [cpp]
                    disabled = false
            })
            .cmd(
                "c++ --version",
                Some(CommandOutput {
                    stdout: String::from(
                        "\
c++ (GCC) 3.3.5
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
            Color::Fixed(149).bold().paint("C++ v3.3.5-g++ ")
        ));
        assert_eq!(expected, actual);

        // What happens with an unknown C++ compiler?
        let actual = ModuleRenderer::new("cpp")
            .config(toml::toml! {
                [cpp]
                    disabled = false
            })
            .cmd(
                "c++ --version",
                Some(CommandOutput {
                    stdout: String::from("HISOFT-C++ Compiler  V1.2\nCopyright © 1984 HISOFT"),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C++ ")));
        assert_eq!(expected, actual);

        // What happens when 'c++ --version' doesn't work, but 'g++ --version' does?
        // This stubs out `c++` but we'll fall back to `g++ --version`
        let actual = ModuleRenderer::new("cpp")
            .config(toml::toml! {
                [cpp]
                    disabled = false
            })
            .cmd("c++ --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C++ v14.2.1-g++ ")
        ));
        assert_eq!(expected, actual);

        // Now with both 'c++' and 'g++' not working, this should fall back to 'clang++ --version'
        let actual = ModuleRenderer::new("cpp")
            .config(toml::toml! {
                [cpp]
                    disabled = false
            })
            .cmd("c++ --version", None)
            .cmd("g++ --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C++ v19.1.7-clang++ ")
        ));
        assert_eq!(expected, actual);

        // What happens when we can't find any of c++, g++ or clang++?
        let actual = ModuleRenderer::new("cpp")
            .config(toml::toml! {
                [cpp]
                    disabled = false
            })
            .cmd("c++ --version", None)
            .cmd("g++ --version", None)
            .cmd("clang++ --version", None)
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

        let actual = ModuleRenderer::new("cpp")
            .config(toml::toml! {
                [cpp]
                    disabled = false
            })
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C++ v14.2.1-g++ ")
        ));

        assert_eq!(expected, actual);
        dir.close()
    }
}
