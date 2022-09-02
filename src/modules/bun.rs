use super::{Context, Module, ModuleConfig};

use crate::configs::bun::BunConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;
use crate::utils::get_command_string_output;

/// Creates a module with the current Bun version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("bun");
    let config = BunConfig::try_load(module.config);

    let is_bun_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_bun_project {
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
                    let bun_version = get_bun_version(context)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &bun_version,
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
            log::warn!("Error in module `bun`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_bun_version(context: &Context) -> Option<String> {
    context
        .exec_cmd("bun", &["--version"])
        .map(get_command_string_output)
        .map(parse_bun_version)
}

fn parse_bun_version(bun_version: String) -> String {
    bun_version.trim_end().to_string()
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_bun_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("bun").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_bun_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("bun.lockb"))?.sync_all()?;
        let actual = ModuleRenderer::new("bun").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint("🍞 v0.1.4 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn no_bun_installed() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("bun.lockb"))?.sync_all()?;
        let actual = ModuleRenderer::new("bun")
            .path(dir.path())
            .cmd("bun --version", None)
            .collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint("🍞 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
