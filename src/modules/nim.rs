use super::{Context, Module, RootModuleConfig, SegmentConfig};

use crate::configs::nim::NimConfig;
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
    let nim_version_output = utils::exec_cmd("nim", &["--version"])?.stdout;
    let nim_version = parse_nim_version(&nim_version_output)?;

    let mut module = context.new_module("nim");
    let config = NimConfig::try_load(module.config);

    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("version", &SegmentConfig::new(nim_version));

    Some(module)
}

fn parse_nim_version(version_cmd_output: &str) -> Option<&str> {
    version_cmd_output
        .lines()
        // First line has the version
        .next()?
        .split(' ')
        .find(|&s| s.chars().all(|c| c >= '0' && c <= '9' || c == '.'))
}

#[cfg(test)]
mod tests {
    use super::parse_nim_version;
    use crate::modules::utils::test::render_module;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn nim_version() {
        let ok_versions = ["1.1.1", "2", "."];
        let not_ok_versions = ["abc", " \n.", ". ", "abc."];

        let all_some = ok_versions.iter().all(|&v| parse_nim_version(v).is_some());
        let all_none = not_ok_versions
            .iter()
            .any(|&v| parse_nim_version(v).is_some());

        assert_eq!(true, all_some);
        assert_eq!(true, all_none);

        let sample_nimc_output = "Nim Compiler Version 1.2.0 [Linux: amd64]
            Compiled at 2020-04-03
            Copyright (c) 2006-2020 by Andreas Rumpf

            git hash: 7e83adff84be5d0c401a213eccb61e321a3fb1ff
            active boot switches: -d:release\n";

        assert_eq!(Some("1.2.0"), parse_nim_version(sample_nimc_output))
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
