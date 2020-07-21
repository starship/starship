use super::{Context, Module, RootModuleConfig};

use crate::configs::zig::ZigConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current Zig version
///
/// Will display the Zig version if any of the following criteria are met:
///     - The current directory contains a file with extension `.zig`
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_zig_project = context
        .try_begin_scan()?
        .set_extensions(&["zig"])
        .is_match();

    if !is_zig_project {
        return None;
    }

    let zig_version = utils::exec_cmd("zig", &["version"])?.stdout;

    let mut module = context.new_module("zig");
    let config = ZigConfig::try_load(module.config);

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
                "version" => Some(Ok(format!("v{}", &zig_version.trim()))),
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
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_zig() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("zig.txt"))?.sync_all()?;
        let actual = render_module("zig", dir.path(), None);
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_zig_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.zig"))?.sync_all()?;
        let actual = render_module("zig", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("↯ v0.6.0")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
