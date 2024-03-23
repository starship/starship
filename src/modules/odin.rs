use super::{Context, Module, ModuleConfig};

use crate::configs::odin::OdinConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Odin version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("odin");
    let config = OdinConfig::try_load(module.config);

    let is_odin_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_odin_project {
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
                    let odin_version = context.exec_cmd("odin", &["version"])?.stdout;
                    let cut_version = odin_version.split(' ').last()?.split(':').next()?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        cut_version.trim(),
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
            log::warn!("Error in module `odin`:\n{}", error);
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
    fn folder_without_odin() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("odin.txt"))?.sync_all()?;
        let actual = ModuleRenderer::new("odin").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_odin_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.odin"))?.sync_all()?;
        let actual = ModuleRenderer::new("odin").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::LightBlue.bold().paint("Ø dev-2024-03 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}