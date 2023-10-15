use super::{Context, Module, ModuleConfig};

use crate::configs::fly::FlyConfig;
use crate::formatter::StringFormatter;
use crate::formatter::VersionFormatter;

/// Creates a module with the current Fly version
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("fly");
    let config = FlyConfig::try_load(module.config);
    let is_fly_project = context
        .try_begin_scan()?
        .set_files(&config.detect_files)
        .is_match();

    if !is_fly_project {
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
                    let fly_version =
                        parse_fly_version(&context.exec_cmd("flyctl", &["version"])?.stdout)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &fly_version,
                        config.version_format,
                    )
                    .map(Ok)
                }
                "app" => {
                    let file_contents = context.read_file_from_pwd(config.detect_files.first()?)?;
                    parse_fly_app_name(file_contents).map(Ok)
                }
                "primary_region" => {
                    let file_contents = context.read_file_from_pwd(config.detect_files.first()?)?;
                    parse_fly_primary_region(file_contents).map(Ok)
                }
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `fly`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn parse_fly_app_name(file_contents: String) -> Option<String> {
    Some({
        let contents = file_contents.parse::<toml::Table>().unwrap();
        let app = contents.get("app")?;

        match app {
            toml::Value::String(app) => app.to_string(),
            _ => "".to_string(),
        }
    })
}

fn parse_fly_primary_region(file_contents: String) -> Option<String> {
    Some({
        let contents = file_contents.parse::<toml::Table>().unwrap();
        let app = contents.get("primary_region")?;

        match app {
            toml::Value::String(app) => app.to_string(),
            _ => "".to_string(),
        }
    })
}

fn parse_fly_version(fly_version: &str) -> Option<String> {
    Some(
        fly_version
            // split into ["flyctl", "v0.1.108"]
            .split_whitespace()
            // return "1.8.3"
            .nth(1)?
            .to_string()
            .replace('v', ""),
    )
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;

    #[test]
    fn folder_without_fly_files() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let actual = ModuleRenderer::new("fly").path(dir.path()).collect();
        let expected = None;
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn folder_with_fly_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("fly.toml"))?.sync_all()?;
        let actual = ModuleRenderer::new("fly").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Green.bold().paint("🎈 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
