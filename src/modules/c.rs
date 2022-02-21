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
                    if config.format.contains("$compiler_name") {
                        // This is ugly, it would be better expressed as "run each
                        //   of these in turn, stopping when one succeeds and gimme
                        //   the results" instead of a mess of nested ifs, but I've
                        //   not yet figured out how to do that in Rust :-)
                        // Try running `cc --version`; if that succeeds, unwrap it.
                        //   otherwise try `gcc --version`; if that succeeds, unwrap it.
                        //   Otherwise try `clang --version` and either unwrap or fail (? op)
                        let c_compiler_info = context.exec_cmd("cc", &["--version"]);
                        let c_compiler_info = if c_compiler_info.is_none() {
                            let c_compiler_info = context.exec_cmd("gcc", &["--version"]);
                            if c_compiler_info.is_none() {
                                context.exec_cmd("clang", &["--version"])?
                            } else {
                                c_compiler_info.unwrap()
                            }
                        } else {
                            c_compiler_info.unwrap()
                        };
                        let c_compiler_info = c_compiler_info.stdout;

                        let c_compiler = if c_compiler_info.contains("clang") {
                            "clang"
                        } else if c_compiler_info.contains("Free Software Foundation") {
                            "gcc"
                        } else {
                            "Unknown compiler"
                        };
                        Some(c_compiler).map(Ok)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .map(|variable| match variable {
                "compiler_version" => {
                    if config.format.contains("$compiler_version") {
                        // Another pile of ugly
                        let c_version = context.exec_cmd("cc", &["-dumpversion"]);
                        let c_version = if c_version.is_none() {
                            let c_version = context.exec_cmd("gcc", &["-dumpversion"]);
                            if c_version.is_none() {
                                context.exec_cmd("clang", &["-dumpversion"])?
                            } else {
                                c_version.unwrap()
                            }
                        } else {
                            c_version.unwrap()
                        };
                        let c_version = c_version.stdout;

                        VersionFormatter::format_module_version(
                            module.get_name(),
                            c_version.trim(),
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

        let actual = ModuleRenderer::new("c").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("C ")
        ));
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
            Color::Fixed(149).bold().paint("C ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
