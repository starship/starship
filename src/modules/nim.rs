use super::{Context, Module, ModuleConfig};

use crate::configs::nim::NimConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Nim version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("nim");
    let config = NimConfig::try_load(module.config);
    let is_nim_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_files)
        .is_match();

    if !is_nim_project {
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
                    .exec_cmd("nim", &["--version"])
                    .map(|command_output| command_output.stdout)
                    .and_then(|nim_version_output| {
                        let nim_version = parse_nim_version(&nim_version_output)?;
                        VersionFormatter::format_module_version(
                            module.get_name(),
                            nim_version,
                            config.version_format,
                        )
                    })
                    .map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
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

fn parse_nim_version(version_cmd_output: &str) -> Option<&str> {
    version_cmd_output
        .lines()
        // First line has the version
        .next()?
        .split(' ')
        .find(|&s| s.chars().all(|c| ('0'..='9').contains(&c) || c == '.'))
}

#[cfg(test)]
mod tests {
    use super::parse_nim_version;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
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

        assert!(all_some);
        assert!(all_none);

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
        let actual = ModuleRenderer::new("nim").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_nimble_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.nimble"))?.sync_all()?;
        let actual = ModuleRenderer::new("nim").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("👑 v1.2.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_nim_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.nim"))?.sync_all()?;
        let actual = ModuleRenderer::new("nim").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("👑 v1.2.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_nims_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("main.nims"))?.sync_all()?;
        let actual = ModuleRenderer::new("nim").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("👑 v1.2.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_cfg_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("cfg.nim"))?.sync_all()?;
        let actual = ModuleRenderer::new("nim").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Yellow.bold().paint("👑 v1.2.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
