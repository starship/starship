use super::{Context, Module, RootModuleConfig};

use crate::configs::red::RedConfig;
use crate::formatter::StringFormatter;

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
                    .map(|output| parse_red_version(output.stdout.trim()))
                    .map(Ok),
                _ => None,
            })
            .parse(None)
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

fn parse_red_version(red_version: &str) -> String {
    format!("v{}", red_version)
}

#[cfg(test)]
mod tests {
    use super::parse_red_version;
    use crate::test::TestRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_red_version() {
        const OUTPUT: &str = "0.6.4\n";
        assert_eq!(parse_red_version(OUTPUT.trim()), "v0.6.4".to_string())
    }

    #[test]
    fn folder_without_red_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = TestRenderer::new().path(dir.path()).module("red");
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_red_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.red"))?.sync_all()?;
        let actual = TestRenderer::new().path(dir.path()).module("red");
        let expected = Some(format!("via {}", Color::Red.bold().paint("🔺 v0.6.4 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_reds_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.reds"))?.sync_all()?;
        let actual = TestRenderer::new().path(dir.path()).module("red");
        let expected = Some(format!("via {}", Color::Red.bold().paint("🔺 v0.6.4 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
