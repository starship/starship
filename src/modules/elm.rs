use super::{Context, Module, ModuleConfig};

use crate::configs::elm::ElmConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Elm version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("elm");
    let config: ElmConfig = ElmConfig::try_load(module.config);

    let is_elm_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_elm_project {
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
                    let elm_version = context.exec_cmd("elm", &["--version"])?.stdout;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        elm_version.trim(),
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
            log::warn!("Error in module `elm`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::{self, File};
    use std::io;

    #[test]
    fn folder_without_elm() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("elm").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_elm_json() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("elm.json"))?.sync_all()?;
        let actual = ModuleRenderer::new("elm").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🌳 v0.19.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_elm_package_json() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("elm-package.json"))?.sync_all()?;
        let actual = ModuleRenderer::new("elm").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🌳 v0.19.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_elm_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".elm-version"))?.sync_all()?;
        let actual = ModuleRenderer::new("elm").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🌳 v0.19.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_elm_stuff_directory() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let elmstuff = dir.path().join("elm-stuff");
        fs::create_dir_all(elmstuff)?;
        let actual = ModuleRenderer::new("elm").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🌳 v0.19.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_elm_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.elm"))?.sync_all()?;
        let actual = ModuleRenderer::new("elm").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Cyan.bold().paint("🌳 v0.19.1 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
