use super::{Context, Module, ModuleConfig};

use crate::configs::hare::HareConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

use once_cell::sync::Lazy;
use std::ops::Deref;

/// Creates a module with the current Hare version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("hare");
    let config = HareConfig::try_load(module.config);
    let is_hare_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_hare_project {
        return None;
    }

    let hare_version =
        Lazy::new(|| parse_hare_version(&context.exec_cmd("hare", &["version"])?.stdout));

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
                    let hare_ver = hare_version.deref().as_ref()?;

                    VersionFormatter::format_module_version(
                        module.get_name(),
                        hare_ver,
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
            log::warn!("Error in module `hare`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_hare_version(hare_stdout: &str) -> Option<String> {
    // hare version output looks like this:
    // Hare dev+SHA

    let version = hare_stdout.split_whitespace().nth(1)?;

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
    fn folder_without_hare_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;

        let actual = ModuleRenderer::new("hare").path(dir.path()).collect();

        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_hare_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.ha"))?.sync_all()?;

        let actual = ModuleRenderer::new("hare").path(dir.path()).collect();

        // let expected = Some(format!("via {}", Color::Cyan.bold().paint("🐰 dev+102a2270 ")));
        let expected = Some(format!(
            "via {}",
            Color::Blue.bold().paint("🐰 dev+102a2270 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_hare_version() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".hare-version"))?.sync_all()?;

        let actual = ModuleRenderer::new("hare").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Blue.bold().paint("🐰 dev+102a2270 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn test_format_hare_version() {
        let input = "Hare dev+102a2270\n\r";
        assert_eq!(parse_hare_version(input), Some("dev+102a2270".to_string()));
    }
}
