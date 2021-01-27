use super::{Context, Module, RootModuleConfig};

use crate::configs::perl::PerlConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current perl version
///
/// Will display the perl version if any of the following criteria are met:
///     - Current directory contains a `.pl`, `.pm` or a `.pod` file
///     - Current directory contains a "Makefile.PL", "Build.PL",  "cpanfile", "cpanfile.snapshot",
///       "META.json", "META.yml", or ".perl-version" file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_perl_project = context
        .try_begin_scan()?
        .set_files(&[
            "Makefile.PL",
            "Build.PL",
            "cpanfile",
            "cpanfile.snapshot",
            "META.json",
            "META.yml",
            ".perl-version",
        ])
        .set_extensions(&["pl", "pm", "pod"])
        .is_match();

    if !is_perl_project {
        return None;
    }

    let mut module = context.new_module("perl");
    let config: PerlConfig = PerlConfig::try_load(module.config);

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
                "version" => {
                    let perl_version =
                        utils::exec_cmd("perl", &["-e", "printf q#%vd#,$^V;"])?.stdout;
                    Some(Ok(format!("v{}", perl_version)))
                }
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `perl`:\n{}", error);
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
    fn folder_without_perl_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("perl").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_makefile_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Makefile.PL"))?.sync_all()?;

        let actual = ModuleRenderer::new("perl").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("🐪 v5.26.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_buildfile_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Build.PL"))?.sync_all()?;

        let actual = ModuleRenderer::new("perl").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("🐪 v5.26.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cpanfile_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("cpanfile"))?.sync_all()?;

        let actual = ModuleRenderer::new("perl").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("🐪 v5.26.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cpanfile_snapshot_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("cpanfile.snapshot"))?.sync_all()?;

        let actual = ModuleRenderer::new("perl").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("🐪 v5.26.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_meta_json_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("META.json"))?.sync_all()?;

        let actual = ModuleRenderer::new("perl").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("🐪 v5.26.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_meta_yml_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("META.yml"))?.sync_all()?;

        let actual = ModuleRenderer::new("perl").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("🐪 v5.26.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_perl_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".perl-version"))?.sync_all()?;

        let actual = ModuleRenderer::new("perl").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("🐪 v5.26.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_perl_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.pl"))?.sync_all()?;

        let actual = ModuleRenderer::new("perl").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("🐪 v5.26.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_perl_module_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.pm"))?.sync_all()?;

        let actual = ModuleRenderer::new("perl").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("🐪 v5.26.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_perldoc_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("any.pod"))?.sync_all()?;

        let actual = ModuleRenderer::new("perl").path(dir.path()).collect();

        let expected = Some(format!(
            "via {}",
            Color::Fixed(149).bold().paint("🐪 v5.26.1 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
