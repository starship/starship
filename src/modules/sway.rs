use super::{Context, Module, ModuleConfig};

use crate::configs::sway::SwayConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Sway version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("sway");
    let config = SwayConfig::try_load(module.config);

    let is_sway_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_sway_project {
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
                    let raw_version = context.exec_cmd("forc", &["--version"])?.stdout;
                    let version = raw_version
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or(&raw_version);

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        version.trim(),
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
            log::warn!("Error in module `sway`:\n{}", error);
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
    fn folder_without_sway() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("sway.txt"))?.sync_all()?;
        let actual = ModuleRenderer::new("sway").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_sway_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.sw"))?.sync_all()?;
        let actual = ModuleRenderer::new("sway").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🌴 v0.24.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_forc_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("Forc.toml"))?.sync_all()?;
        let actual = ModuleRenderer::new("sway").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🌴 v0.24.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
