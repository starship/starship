use std::{borrow::Cow, ops::Deref, sync::LazyLock};

use semver::Version;

use crate::{
    config::ModuleConfig,
    configs::fortran::FortranConfig,
    formatter::{StringFormatter, VersionFormatter},
};

use super::{Context, Module};

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("fortran");
    let config = FortranConfig::try_load(module.config);

    let is_fortran_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_fortran_project {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        let compiler_info = LazyLock::new(|| context.exec_cmds_return_first(config.commands));

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
                    let compiler_info = &compiler_info.deref().as_ref()?.stdout;

                    let compiler = if compiler_info.contains("GNU") {
                        "gfortran"
                    } else if compiler_info.contains("flang-new") {
                        "flang-new"
                    } else {
                        return None;
                    };
                    Some(Ok(Cow::Borrowed(compiler)))
                },
                "version" => {
                    let compiler_info = &compiler_info.deref().as_ref()?.stdout;

                    VersionFormatter::format_module_version(
                        &module.get_name(),
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
            log::warn!("Error in module `fortran`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io};

    use nu_ansi_term::Color;

    use crate::test::ModuleRenderer;

    #[test]
    fn folder_without_fortran_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("fortran").path(dir.path()).collect();

        let expected = None;

        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn folder_with_f_fortran_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("test.f"))?.sync_all()?;

        let actual = ModuleRenderer::new("fortran").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Purple.bold().paint("󱈚 14.2.0 ")));

        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn folder_with_capital_f18_fortran_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("test.F18"))?.sync_all()?;

        let actual = ModuleRenderer::new("fortran").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Purple.bold().paint("󱈚 14.2.0 ")));

        assert_eq!(expected, actual);

        dir.close()
    }

    #[test]
    fn folder_with_fpm_config_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("fpm.toml"))?.sync_all()?;

        let actual = ModuleRenderer::new("fortran").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Purple.bold().paint("󱈚 14.2.0 ")));

        assert_eq!(expected, actual);

        dir.close()
    }
}
