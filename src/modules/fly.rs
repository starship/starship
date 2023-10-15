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
                "app" => get_toml_value(context, "app").map(Ok),
                "primary_region" => get_toml_value(context, "primary_region").map(Ok),
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

fn get_toml_value(context: &Context<'_>, key: &str) -> Option<String> {
    context
        .read_file_from_pwd("fly.toml")
        .and_then(|contents| match contents.parse::<toml::Table>() {
            Ok(dict) => {
                let val = dict.get(key)?;
                match val {
                    toml::Value::String(x) => Some(x.to_string()),
                    _ => None,
                }
            }
            Err(_) => None,
        })
        .or(None)
}

fn parse_fly_version(fly_version: &str) -> Option<String> {
    Some(
        fly_version
            // split into ["flyctl", "v0.1.108"]
            .split_whitespace()
            .nth(1)?
            // "v0.1.108"
            .to_string()
            // return "0.1.108"
            .replace('v', ""),
    )
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;
    use std::fs::File;
    use std::io;
    use std::io::Write;

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
        let expected = Some(format!("via {}", Color::Purple.bold().paint("🎈 ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn fly_toml_with_app_name() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("fly.toml"))?;
        file.write_all(b"app = \"my-fly-app-1234\"")?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("fly").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Purple.bold().paint("🎈 my-fly-app-1234 ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn fly_toml_with_region() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("fly.toml"))?;
        file.write_all(b"primary_region = \"bos\"")?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("fly").path(dir.path()).collect();
        let expected = Some(format!("via {}", Color::Purple.bold().paint("🎈 bos ")));
        assert_eq!(expected, actual);
        dir.close()
    }

    #[test]
    fn fly_toml_with_app_and_region() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let mut file = File::create(dir.path().join("fly.toml"))?;
        file.write_all(
            b"
app = \"my-fly-app-1234\"
primary_region = \"bos\"
        ",
        )?;
        file.sync_all()?;

        let actual = ModuleRenderer::new("fly").path(dir.path()).collect();
        let expected = Some(format!(
            "via {}",
            Color::Purple.bold().paint("🎈 my-fly-app-1234 bos ")
        ));
        assert_eq!(expected, actual);
        dir.close()
    }
}
