use super::{Context, Module, RootModuleConfig};

use crate::configs::haskell::HaskellConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Haskell version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("haskell");
    let config = HaskellConfig::try_load(module.config);
    let is_haskell_project = context
        .try_begin_scan()?
        .set_extensions(&config.detect_extensions)
        .is_match();

    if !is_haskell_project {
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
                    .exec_cmd("ghc", &["--version"])
                    .and_then(|output| parse_ghc_version(output.stdout.trim()))
                    .map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `haskell`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_ghc_version(ghc_version: &str) -> Option<String> {
    let version = ghc_version.split_whitespace().nth(7)?;

    Some(format!("v{}", version))
}

#[cfg(test)]
mod tests {
    use super::parse_ghc_version;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_ghc_version() {
        const OUTPUT: &str = "The Glorious Glasgow Haskell Compilation System, version 8.6.5\n";
        assert_eq!(parse_ghc_version(OUTPUT.trim()), Some("v8.6.5".to_string()))
    }

    #[test]
    fn folder_without_haskell_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_hs_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.hs"))?.sync_all()?;
        let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Purple.bold().paint(" v8.6.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_hs_boot_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.hs-boot"))?.sync_all()?;
        let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Purple.bold().paint(" v8.6.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_hsc_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.hsc"))?.sync_all()?;
        let actual = ModuleRenderer::new("haskell").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Purple.bold().paint(" v8.6.5 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
