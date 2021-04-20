use super::{Context, Module, RootModuleConfig};

use crate::configs::dlang::DLangConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current D version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("dlang");
    let config = DLangConfig::try_load(module.config);
    let is_dlang_project = context
        .try_begin_scan()?
        .set_extensions(&config.detect_extensions)
        .set_files(&config.detect_files)
        .is_match();

    if !is_dlang_project {
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
                    .exec_cmd("dmd", &["--version"])
                    .and_then(|output| parse_dlang_version(output.stdout.trim()))
                    .map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `dlang`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_dlang_version(dlang_version: &str) -> Option<String> {
    let version = dlang_version
        // split into ["DMD32 D Compiler v2.096.0-dirty", "Copyright (C) 1999-2021 by The D Language Foundation, All Rights Reserved written by Walter Bright"]
        .lines()
        // get something along the lines of "DMD32 D Compiler v2.096.0-dirty"
        .next()?
        // split into ["DMD32", "D", "Compiler", "v2.096.0-dirty"]
        .split_whitespace()
        // return v2.096.0-dirty
        .nth(3)?;

    Some(version.to_owned())
}

#[cfg(test)]
mod tests {
    use super::parse_dlang_version;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_dlang_version() {
        const OUTPUT: &str = "DMD32 D Compiler v2.096.0-dirty\nCopyright (C) 1999-2021 by The D Language Foundation, All Rights Reserved written by Walter Bright\n";
        assert_eq!(
            parse_dlang_version(OUTPUT.trim()),
            Some("v2.096.0-dirty".to_string())
        )
    }

    #[test]
    fn folder_without_dlang_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("dlang").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_d_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.d"))?.sync_all()?;
        let actual = ModuleRenderer::new("dlang").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::RGB(176, 57, 49).paint(" v2.096.0-dirty ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_di_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.di"))?.sync_all()?;
        let actual = ModuleRenderer::new("dlang").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::RGB(176, 57, 49).paint(" v2.096.0-dirty ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dub_json_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("dub.json"))?.sync_all()?;
        let actual = ModuleRenderer::new("dlang").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::RGB(176, 57, 49).paint(" v2.096.0-dirty ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_dub_sdl_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("dub.sdl"))?.sync_all()?;
        let actual = ModuleRenderer::new("dlang").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::RGB(176, 57, 49).paint(" v2.096.0-dirty ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
