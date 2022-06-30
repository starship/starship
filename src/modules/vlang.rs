use super::{Context, Module, ModuleConfig};

use crate::configs::v::VConfig;
use crate::formatter::{StringFormatter, VersionFormatter};

/// Creates a module with the current V version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("vlang");
    let config = VConfig::try_load(module.config);
    let is_v_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .set_extensions(&config.detect_extensions)
        .set_folders(&config.detect_folders)
        .is_match();

    if !is_v_project {
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
                    .exec_cmd("v", &["version"])
                    .map(|output| parse_v_version(&output.stdout))?
                    .map(|output| {
                        VersionFormatter::format_module_version(
                            module.get_name(),
                            &output,
                            config.version_format,
                        )
                    })?
                    .map(Ok),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `vlang`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_v_version(v_version: &str) -> Option<String> {
    let version = v_version
        // split into ["V", "0.2", "30c0659"]
        .split_whitespace()
        // return "0.2"
        .nth(1)?;

    Some(version.to_owned())
}

#[cfg(test)]
mod tests {
    use super::parse_v_version;
    use crate::test::ModuleRenderer;
    use ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn test_parse_v_version() {
        const OUTPUT: &str = "V 0.2 30c0659\n";
        assert_eq!(parse_v_version(OUTPUT), Some("0.2".to_string()))
    }

    #[test]
    fn folder_without_v_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("vlang").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_v_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.v"))?.sync_all()?;
        let actual = ModuleRenderer::new("vlang").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("V v0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_vmod_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("v.mod"))?.sync_all()?;
        let actual = ModuleRenderer::new("vlang").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("V v0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_vpkg_file() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("vpkg.json"))?.sync_all()?;
        let actual = ModuleRenderer::new("vlang").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("V v0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_vpkg_lockfile() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join(".vpkg-lock.json"))?.sync_all()?;
        let actual = ModuleRenderer::new("vlang").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("V v0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn version_formatting() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("hello.v"))?.sync_all()?;
        let actual = ModuleRenderer::new("vlang")
            .path(dir.path())
            .config(toml::toml! {
                [vlang]
                version_format = "${raw}"
            })
            .collect();
        let expected = Some(format!("via {}", Color::Blue.bold().paint("V 0.2 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
