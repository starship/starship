use super::{Context, Module, RootModuleConfig};

use crate::configs::c::CConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current c version
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
        let c_compiler_info = if config.format.contains("$compiler_name")
            || config.format.contains("$compiler_version")
        {
            context.exec_cmds_return_first(&[
                // the compiler is usually cc, and --version works on gcc and clang
                &["cc", "--version"],
                // but on some platforms gcc is installed as *gcc*, not cc
                &["gcc", "--version"],
                // for completeness, although I've never seen a clang that wasn't cc
                &["clang", "--version"],
            ])
        } else {
            None
        };

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
                "compiler_name" => {
                    let c_compiler_info = &c_compiler_info.as_ref()?.stdout;

                    let c_compiler = if c_compiler_info.contains("clang") {
                        "clang"
                    } else if c_compiler_info.contains("Free Software Foundation") {
                        "gcc"
                    } else {
                        "Unknown compiler"
                    };
                    Some(c_compiler).map(Ok)
                }
                _ => None,
            })
            .map(|variable| match variable {
                "compiler_version" => {
                    if config.format.contains("$compiler_version") {
                        // Clang says ...
                        //   Apple clang version 13.0.0 ...\n
                        //   OpenBSD clang version 11.1.0\n...
                        //   FreeBSD clang version 11.0.1 ...\n
                        // so seems to have the version always in position 3.
                        // gcc says ...
                        //   gcc (OmniOS 151036/9.3.0-il-1) 9.3.0\n...
                        //   gcc (Debian 10.2.1-6) 10.2.1 ...\n
                        // so again in position 3.
                        // But this is CRUFTY AS HELL
                        VersionFormatter::format_module_version(
                            module.get_name(),
                            &c_compiler_info
                                .as_ref()?
                                .stdout
                                .split_whitespace()
                                .nth(3)?
                                .to_string(),
                            config.version_format,
                        )
                        .map(Ok)
                    } else {
                        None
                    }
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
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
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

        // FIXME voodoo here to set up a mock for 'cc --version' claiming to be gcc
        // NB the test for when it claims to be clang is in the ..._with_h_file test
        let actual = ModuleRenderer::new("c").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C gcc v10.2.1")));
        assert_eq!(expected, actual);

        // FIXME voodoo here to set up a mock for 'cc --version' claiming to be something
        // unknown:
        // HISOFT-C Compiler  V1.2
        // Copyright © 1984 HISOFT
        let actual = ModuleRenderer::new("c").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C Unknown compiler")));
        assert_eq!(expected, actual);

        // FIXME voodoo here to set up a mock for 'cc --version' not working,
        // then 'gcc --version' returning something sensible
        let actual = ModuleRenderer::new("c").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C gcc v10.2.1")));
        assert_eq!(expected, actual);

        // FIXME voodoo here to set up a mock for 'cc --version' not working,
        // then 'gcc --version' not working, then 'clang --version' returning
        // something sensible
        let actual = ModuleRenderer::new("c").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C clang v11.0.1")));
        assert_eq!(expected, actual);

        // FIXME voodoo here to set up a mock for 'cc --version' not working,
        // then 'gcc --version' not working, then 'clang --version' not working
        let actual = ModuleRenderer::new("c").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Fixed(149).bold().paint("C ")));
        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn folder_with_h_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.h"))?.sync_all()?;

        let actual = ModuleRenderer::new("c").path(dir.path()).collect();
        let expected = Some(format!("using {}", Color::Fixed(149).bold().paint("C clang v11.0.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
