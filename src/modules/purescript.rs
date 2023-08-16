use super::{Context, Module, ModuleConfig};

use crate::configs::purescript::PureScriptConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current PureScript version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("purescript");
    let config: PureScriptConfig = PureScriptConfig::try_load(module.config);
    let is_purs_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_folders(&config.detect_folders)
        .set_extensions(&config.detect_extensions)
        .is_match();

    if !is_purs_project {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "version" => {
                    let purs_version = context.exec_cmd("purs", &["--version"])?.stdout;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        purs_version.trim(),
                        config.version_format,
                    )
                    .map(Ok)
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `purescript`:\n{}", error);
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
    fn folder_without_purescript_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("purescript").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_purescript_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Main.purs"))?.sync_all()?;

        let actual = ModuleRenderer::new("purescript").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::White.bold().paint("<=> v0.13.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_spago_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("spago.dhall"))?.sync_all()?;

        let actual = ModuleRenderer::new("purescript").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::White.bold().paint("<=> v0.13.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
