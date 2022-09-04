use super::{Context, Module, ModuleConfig};
use crate::formatter::VersionFormatter;

use crate::configs::rlang::RLangConfig;
use crate::formatter::StringFormatter;
use crate::utils::get_command_string_output;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("rlang");
    let config: RLangConfig = RLangConfig::try_load(module.config);

    let is_r_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();
    if !is_r_project {
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
                    let r_version_string =
                        get_command_string_output(context.exec_cmd("R", &["--version"])?);
                    let r_version = parse_r_version(&r_version_string)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &r_version,
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
            log::warn!("Error in module `rlang`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_r_version(r_version: &str) -> Option<String> {
    r_version
        .lines()
        // take first line
        .next()?
        // split into ["R", "version", "3.6.3", "(2020-02-29)", ...]
        .split_whitespace()
        // and pick version entry at index 2, i.e. "3.6.3".
        .nth(2)
        .map(ToString::to_string)
}

#[cfg(test)]
mod tests {
    use super::parse_r_version;
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_r_version() {
        let r_v3 = r#"R version 4.1.0 (2021-05-18) -- "Camp Pontanezen"
Copyright (C) 2021 The R Foundation for Statistical Computing
Platform: x86_64-w64-mingw32/x64 (64-bit)\n

R is free software and comes with ABSOLUTELY NO WARRANTY.
You are welcome to redistribute it under the terms of the
GNU General Public License versions 2 or 3.
For more information about these matters see
https://www.gnu.org/licenses/."#;
        assert_eq!(parse_r_version(r_v3), Some(String::from("4.1.0")));
    }

    #[test]
    fn folder_with_r_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("analysis.R"))?.sync_all()?;
        check_r_render(&dir);
        dir.close()
    }

    #[test]
    fn folder_with_rd_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("analysis.Rd"))?.sync_all()?;
        check_r_render(&dir);
        dir.close()
    }

    #[test]
    fn folder_with_rmd_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("analysis.Rmd"))?.sync_all()?;
        check_r_render(&dir);
        dir.close()
    }

    #[test]
    fn folder_with_rproj_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("analysis.Rproj"))?.sync_all()?;
        check_r_render(&dir);
        dir.close()
    }

    #[test]
    fn folder_with_rsx_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("analysis.Rsx"))?.sync_all()?;
        check_r_render(&dir);
        dir.close()
    }

    #[test]
    fn folder_with_rprofile_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".Rprofile"))?.sync_all()?;
        check_r_render(&dir);
        dir.close()
    }

    #[test]
    fn folder_with_rproj_user_folder() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let rprofile = dir.path().join(".Rproj.user");
        fs::create_dir_all(&rprofile)?;
        check_r_render(&dir);
        dir.close()
    }

    fn check_r_render(dir: &tempfile::TempDir) {
        let actual = ModuleRenderer::new("rlang").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("📐 v4.1.0 ")));
        assert_eq!(expected, actual);
    }
}
