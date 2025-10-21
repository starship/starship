use super::{Context, Module, ModuleConfig};

use crate::configs::pixi::PixiConfig;
use crate::formatter::{StringFormatter, VersionFormatter};
use crate::utils::get_command_string_output;

/// Creates a module with the current Pixi environment
///
/// Will display the Pixi environment iff `$PIXI_ENVIRONMENT_NAME` is set.
/// Will display the Pixi version iff pixi files are detected or `$PIXI_ENVIRONMENT_NAME` is set.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("pixi");
    let config: PixiConfig = PixiConfig::try_load(module.config);

    let pixi_environment_name = context.get_env("PIXI_ENVIRONMENT_NAME");
    let is_pixi_project = pixi_environment_name.is_some()
        || context
            .try_begin_scan()?
            .set_files(&config.detect_files)
            .set_extensions(&config.detect_extensions)
            .set_folders(&config.detect_folders)
            .is_match();
    if !is_pixi_project {
        return None;
    }

    let pixi_environment_name = if !config.show_default_environment
        && pixi_environment_name == Some("default".to_string())
    {
        None
    } else {
        pixi_environment_name
    };

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|variable, _| match variable {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "environment" => pixi_environment_name.clone().map(Ok),
                "version" => {
                    let pixi_version = get_pixi_version(context, &config)?;
                    VersionFormatter::format_module_version(
                        module.get_name(),
                        &pixi_version,
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
            log::warn!("Error in module `pixi`:\n{error}");
            return None;
        }
    });

    Some(module)
}

fn get_pixi_version(context: &Context, config: &PixiConfig) -> Option<String> {
    let version = config
        .pixi_binary
        .0
        .iter()
        .find_map(|binary| context.exec_cmd(binary, &["--version"]))
        .map(get_command_string_output)?;

    Some(version.split_once(' ')?.1.trim().to_string())
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io};

    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn not_in_env() {
        let actual = ModuleRenderer::new("pixi").collect();

        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn ignore_default_environment() {
        let actual = ModuleRenderer::new("pixi")
            .env("PIXI_ENVIRONMENT_NAME", "default")
            .config(toml::toml! {
                [pixi]
                show_default_environment = false
            })
            .collect();

        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🧚 v0.33.0 ")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn env_set() {
        let actual = ModuleRenderer::new("pixi")
            .env("PIXI_ENVIRONMENT_NAME", "py312")
            .collect();

        let expected = Some(format!(
            "via {}",
            Color::Yellow.bold().paint("🧚 v0.33.0 (py312) ")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn folder_with_pixi_toml() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        File::create(dir.path().join("pixi.toml"))?.sync_all()?;

        let actual = ModuleRenderer::new("pixi").path(dir.path()).collect();

        let expected = Some(format!("via {}", Color::Yellow.bold().paint("🧚 v0.33.0 ")));
        assert_eq!(expected, actual);
        dir.close()
    }
}
