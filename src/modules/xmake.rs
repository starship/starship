use super::{Context, Module, ModuleConfig};

use crate::configs::xmake::XMakeConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

/// Creates a module with the current `XMake` version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("xmake");
    let config = XMakeConfig::try_load(module.config);

    let is_xmake_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_xmake_project {
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
                    let xmake_version =
                        parse_xmake_version(&context.exec_cmd("xmake", &["--version"])?.stdout)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &xmake_version,
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
            log::warn!("Error in module `xmake`: \n{error}");
            return None;
        }
    });

    Some(module)
}

fn parse_xmake_version(xmake_version: &str) -> Option<String> {
    Some(
        xmake_version.
        // split into ["xmake", "v3.0.0+HEAD.0db4fe6", ".."]
        split_whitespace().
        // get "v3.0.0+HEAD.0db4fe6"
        nth(1)?.
        // remove the "v" prefix
        trim_start_matches('v').
        // remove "+HEAD.0db4fe6" suffix
        split('+').next()?.
        to_string(),
    )
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_xmake_lists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("xmake").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_xmake_lists() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("xmake.lua"))?.sync_all()?;
        let actual = ModuleRenderer::new("xmake").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("△ v2.9.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
