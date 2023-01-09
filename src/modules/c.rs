use super::{Context, Module, ModuleConfig};

use crate::configs::c::CConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

use once_cell::sync::Lazy;
use semver::Version;
use std::borrow::Cow;
use std::ops::Deref;

/// Creates a module with the current C compiler and version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("c");
    let config: CConfig = CConfig::try_load(module.config);
    let is_c_project = context
        .try_begin_scan()?
        .set_extensions(&config.detect_extensions)
        .set_files(&config.detect_files)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_c_project {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        let c_compiler_info = Lazy::new(|| context.exec_cmds_return_first(config.commands));

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
                    let c_compiler_info = &c_compiler_info.deref().as_ref()?.stdout;

                    let c_compiler = if c_compiler_info.contains("clang") {
                        "clang"
                    } else if c_compiler_info.contains("Free Software Foundation") {
                        "gcc"
                    } else {
                        return None;
                    };
                    Some(c_compiler).map(Cow::Borrowed).map(Ok)
                }
                "version" => {
                    let c_compiler_info = &c_compiler_info.deref().as_ref()?.stdout;

                    // Clang says ...
                    //   Apple clang version 13.0.0 ...\n
                    //   OpenBSD clang version 11.1.0\n...
                    //   FreeBSD clang version 11.0.1 ...\n
                    // so we always want the first semver-ish whitespace-
                    // separated "word".
                    // gcc says ...
                    //   gcc (OmniOS 151036/9.3.0-il-1) 9.3.0\n...
                    //   gcc (Debian 10.2.1-6) 10.2.1 ...\n
                    //   cc (GCC) 3.3.5 (Debian 1:3.3.5-13)\n...
                    // so again we always want the first semver-ish word.
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        c_compiler_info
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
            log::warn!("Error in module `c`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::{test::ModuleRenderer, utils::CommandOutput};
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_c_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("c").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_c_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.c"))?.sync_all()?;

        // What happens when `cc --version` says it's modern gcc?
        // The case when it claims to be clang is covered in folder_with_h_file,
        // and uses the mock in src/test/mod.rs.
        let actual = ModuleRenderer::new("c")
            .cmd(
                "cc --version",
                Some(CommandOutput {
                    stdout: String::from(
                        "\
cc (Debian 10.2.1-6) 10.2.1 20210110
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
            Color::Fixed(149).bold().paint("C v10.2.1-gcc ")
        ));
        assert_eq!(expected, actual);

        // What happens when `cc --version` says it's ancient gcc?
        let actual = ModuleRenderer::new("c")
            .cmd(
                "cc --version",
                Some(CommandOutput {
                    stdout: String::from(
                        "\
cc (GCC) 3.3.5 (Debian 1:3.3.5-13)
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
            Color::Fixed(149).bold().paint("C v3.3.5-gcc ")
        ));
        assert_eq!(expected, actual);

        // What happens with an unknown C compiler? Needless to say, we're
        // not running on a Z80 so we're never going to see this one in reality!
        let actual = ModuleRenderer::new("c")
            .cmd(
                "cc --version",
                Some(CommandOutput {
                    stdout: String::from("HISOFT-C Compiler  V1.2\nCopyright © 1984 HISOFT"),
                    stderr: String::default(),
                }),
            )
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C ")));
        assert_eq!(expected, actual);

        // What happens when 'cc --version' doesn't work, but 'gcc --version' does?
        // This stubs out `cc` but we'll fall back to `gcc --version` as defined in
        // src/test/mod.rs.
        // Also we don't bother to redefine the config for this one, as we've already
        // proved we can parse its version.
        let actual = ModuleRenderer::new("c")
            .cmd("cc --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C v10.2.1-gcc ")
        ));
        assert_eq!(expected, actual);

        // Now with both 'cc' and 'gcc' not working, this should fall back to 'clang --version'
        let actual = ModuleRenderer::new("c")
            .cmd("cc --version", None)
            .cmd("gcc --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C v11.1.0-clang ")
        ));
        assert_eq!(expected, actual);

        // What happens when we can't find any of cc, gcc or clang?
        let actual = ModuleRenderer::new("c")
            .cmd("cc --version", None)
            .cmd("gcc --version", None)
            .cmd("clang --version", None)
            .path(dir.path())
            .collect();
        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C ")));
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn folder_with_h_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.h"))?.sync_all()?;

        let actual = ModuleRenderer::new("c").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C v11.0.1-clang ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
