use super::{Context, Module, RootModuleConfig};

use crate::configs::r::RConfig;
use crate::formatter::StringFormatter;

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("r");
    let config: RConfig = RConfig::try_load(module.config);

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
                "version" => get_r_version(context).map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `r`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_r_version(context: &Context) -> Option<String> {
    let r_version = context.exec_cmd("R", &["--version"])?.stderr;
    parse_version(&r_version)
}

fn parse_version(r_version: &str) -> Option<String> {
    let version = r_version
        .lines()
        // take first line
        .next()
        // split into ["R", "version", "3.6.3", "(2020-02-29)", ...]
        // and pick version entry at index 2, i.e. "3.6.3".
        .and_then(|s| s.split_whitespace().nth(2));

    version.map(|ver| format!("v{}", ver))
}

#[cfg(test)]
mod tests {
    // use crate::modules::utils::test::render_module;
    // use ansi_term::Color;
    // use std::fs::{self, File};
    // use std::io;
    // use tempfile;
    use super::*;

    #[test]
    fn test_parse_r_version() {
        let r_v3 = r#"R version 3.6.3 (2020-02-29) -- "Holding the Windsock"
        Copyright (C) 2020 The R Foundation for Statistical Computing
        Platform: x86_64-w64-mingw32/x64 (64-bit)
        
        R is free software and comes with ABSOLUTELY NO WARRANTY.
        You are welcome to redistribute it under the terms of the
        GNU General Public License versions 2 or 3.
        For more information about these matters see
        https://www.gnu.org/licenses/."#;
        assert_eq!(parse_version(r_v3), Some(String::from("v3.6.3")));
    }

    #[test]
    fn test_parse_r_invalid_semantic_version() {
        let r_invalid = r#"r_version=R version 3.6.5.2 (2020-02-29) -- "Holding the Windsock"
        Copyright (C) 2020 The R Foundation for Statistical Computing
        Platform: x86_64-w64-mingw32/x64 (64-bit)
        
        R is free software and comes with ABSOLUTELY NO WARRANTY.
        You are welcome to redistribute it under the terms of the
        GNU General Public License versions 2 or 3.
        For more information about these matters see
        https://www.gnu.org/licenses/."#;
        assert_eq!(parse_version(r_invalid), None);
    }
}
