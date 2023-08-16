use super::{Context, Module, ModuleConfig};

use crate::configs::red::RedConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

/// Creates a module with the current  Red version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("red");
    let config = RedConfig::try_load(module.config);
    let is_red_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_red_project {
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
                "version" => context
                    .exec_cmd("red", &["--version"])
                    .map(|output| {
                        VersionFormatter::format_module_version(
                            module.get_name(),
                            output.stdout.trim(),
                            config.version_format,
                        )
                    })?
                    .map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `red`:\n{}", error);
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
    fn folder_without_red_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("red").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_red_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.red"))?.sync_all()?;
        let actual = ModuleRenderer::new("red").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint("🔺 v0.6.4 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_reds_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.reds"))?.sync_all()?;
        let actual = ModuleRenderer::new("red").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint("🔺 v0.6.4 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn version_formatting() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.red"))?.sync_all()?;
        let actual = ModuleRenderer::new("red")
            .path(dir.path())
            .config(toml::toml! {
                [red]
                version_format = "${raw}"
            })
            .collect();
        let expected = Some(format!("via {}", Color::Red.bold().paint("🔺 0.6.4 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
