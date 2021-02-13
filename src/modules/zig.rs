use super::{Context, Module, RootModuleConfig};

use crate::configs::zig::ZigConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Zig version
///
/// Will display the Zig version if any of the following criteria are met:
///     - The current directory contains a file with extension `.zig`
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("zig");
    let config = ZigConfig::try_load(module.config);

    let is_zig_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_zig_project {
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
                    let zig_version_output = context.exec_cmd("zig", &["version"])?.stdout;
                    let zig_version = format!("v{}", zig_version_output.trim());
                    Some(Ok(zig_version))
                }
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `zig`:\n{}", error);
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
    fn folder_without_zig() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("zig.txt"))?.sync_all()?;
        let actual = ModuleRenderer::new("zig").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_zig_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.zig"))?.sync_all()?;
        let actual = ModuleRenderer::new("zig").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("↯ v0.6.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
