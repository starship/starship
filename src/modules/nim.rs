use super::{Context, Module, RootModuleConfig};

use crate::configs::nim::NimConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the current Nim version
///
/// Will display the Nim version if any of the following criteria are met:
///     - The current directory contains a file with extension `.nim`, `.nims`, or `.nimble`
///     - The current directory contains a `nim.cfg` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_nim_project = context
        .try_begin_scan()?
        .set_files(&["nim.cfg"])
        .set_extensions(&["nim", "nims", "nimble"])
        .is_match();

    if !is_nim_project {
        return None;
    }

    let nim_version = utils::exec_cmd("nim", &["--version"])?.stdout;

    let mut module = context.new_module("nim");
    let config = NimConfig::try_load(module.config);
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
                "version" => parse_nim_version(&nim_version).map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `nim`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_nim_version(nim_version: &str) -> Option<String> {
    let version = nim_version
        // split into ["Nim", "Compiler", "Version", "1.2.0", ...]
        .split_whitespace()
        // return "1.2.0"
        .nth(3)?;

    Some(format!("v{}", version))
}

#[cfg(test)]
mod tests {
    use super::parse_nim_version;
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_nim_version() {
        const OUTPUT: &str = "\
Nim Compiler Version 1.2.0 [Linux: amd64]
Compiled at 2020-04-03
Copyright (c) 2006-2020 by Andreas Rumpf

git hash: 7e83adff84be5d0c401a213eccb61e321a3fb1ff
active boot switches: -d:release\n";

        assert_eq!(Some("v1.2.0".to_string()), parse_nim_version(OUTPUT))
    }

    #[test]
    fn folder_without_nim() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("nim.txt"))?.sync_all()?;
        let actual = render_module("nim", dir.path(), None);
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_nimble_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.nimble"))?.sync_all()?;
        let actual = render_module("nim", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("👑 v1.2.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_nim_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.nim"))?.sync_all()?;
        let actual = render_module("nim", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("👑 v1.2.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_nims_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.nims"))?.sync_all()?;
        let actual = render_module("nim", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("👑 v1.2.0")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cfg_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("cfg.nim"))?.sync_all()?;
        let actual = render_module("nim", dir.path(), None);
        let expected = Some(format!("via {} ", Color::Yellow.bold().paint("👑 v1.2.0")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
