use super::{Context, Module, RootModuleConfig};

use crate::configs::deno::DenoConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Deno version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("deno");
    let config = DenoConfig::try_load(module.config);
    let is_deno_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .is_match();

    if !is_deno_project {
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
                    let deno_version = context.exec_cmd("deno", &["-V"])?.stdout;
                    format_deno_version(&deno_version, config.version_format).map(Ok)
                }
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `deno`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn format_deno_version(deno_version: &str, version_format: &str) -> Option<String> {
    let version = deno_version
        // split into ["deno", "1.8.3"]
        .split_whitespace()
        // return "1.8.3"
        .nth(1)?;

    match VersionFormatter::format_version(version, version_format) {
        Ok(formatted) => Some(formatted),
        Err(error) => {
            log::warn!("Error formatting `deno` version:\n{}", error);
            Some(format!("v{}", version))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::format_deno_version;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_format_deno_version() {
        const OUTPUT: &str = "deno 1.8.3\n";
        assert_eq!(
            format_deno_version(OUTPUT.trim(), "v${major}.${minor}.${patch}"),
            Some("v1.8.3".to_string())
        )
    }

    #[test]
    fn test_format_deno_version_truncated() {
        assert_eq!(
            format_deno_version("deno 1.8.3\n", "v${major}.${minor}"),
            Some("v1.8".to_string())
        );
    }

    #[test]
    fn test_format_deno_version_is_malformed() {
        assert_eq!(
            format_deno_version("deno 1.8\n", "v${major}.${minor}.${patch}"),
            Some("v1.8.".to_string())
        );
    }

    #[test]
    fn folder_without_deno_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("deno").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mod_ts() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mod.ts"))?.sync_all()?;
        let actual = ModuleRenderer::new("deno").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🦕 v1.8.3 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_mod_js() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("mod.js"))?.sync_all()?;
        let actual = ModuleRenderer::new("deno").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🦕 v1.8.3 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_deps_ts() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("deps.ts"))?.sync_all()?;
        let actual = ModuleRenderer::new("deno").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🦕 v1.8.3 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_deps_js() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("deps.js"))?.sync_all()?;
        let actual = ModuleRenderer::new("deno").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🦕 v1.8.3 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
