use super::{Context, Module, ModuleConfig};

use crate::configs::fennel::FennelConfig;
use crate::formatter::{StringFormatter, VersionFormatter};
use crate::utils::get_command_string_output;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("fennel");
    let config = FennelConfig::try_load(module.config);

    let is_fnl_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extentions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_fnl_project {
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
                "version" => {
                    let fennel_version_string =
                        get_command_string_output(context.exec_cmd("fennel", &["--version"])?);
                    let fennel_version = parse_fennel_version(&fennel_version_string)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &fennel_version,
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
            log::warn!("Error in module `fennel`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_fennel_version(fennel_version: &str) -> Option<String> {
    // fennel -v output looks like this:
    // Fennel 1.2.1 on PUC Lua 5.4
    let version = fennel_version
        // split into ["Fennel", "1.2.1", "on", ...]
        .split_whitespace()
        // take "1.2.1"
        .nth(1)?;

    Some(version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_fennel_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("fennel").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_fennel_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("man.fnl"))?.sync_all()?;
        let actual = ModuleRenderer::new("fennel").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🧅 v1.2.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_parse_fennel_version() {
        let fennel_input = "Fennel 1.2.1 on PUC Lua 5.4";
        assert_eq!(
            parse_fennel_version(fennel_input),
            Some("1.2.1".to_string())
        );
    }
}
