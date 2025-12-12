use super::{Context, Module, ModuleConfig};

use crate::configs::mise::MiseConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current mise config
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("mise");
    let config = MiseConfig::try_load(module.config);

    let mise_applies = !config.disabled
        && context
            .try_begin_scan()?
            .set_extensions(&config.detect_extensions)
            .set_files(&config.detect_files)
            .set_folders(&config.detect_folders)
            .is_match();

    if !mise_applies {
        return None;
    }

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "symbol" => Some(Ok(config.symbol)),
                "health" => match context.exec_cmd("mise", &["doctor"]) {
                    Some(_) => Some(Ok(config.healthy_symbol)),
                    None => Some(Ok(config.unhealthy_symbol)),
                },
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(e) => {
            log::warn!("{e}");

            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use crate::utils::CommandOutput;
    use nu_ansi_term::Color;

    use std::io;

    #[test]
    fn folder_without_mise_config() {
        let renderer = ModuleRenderer::new("mise").config(toml::toml! {
            [mise]
            disabled = false
        });

        assert_eq!(None, renderer.collect());
    }

    #[test]
    fn folder_with_mise_config_file_healthy() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
            })
            .cmd(
                "mise doctor",
                Some(CommandOutput {
                    stdout: String::default(),
                    stderr: String::default(),
                }),
            );

        let expected = Some(format!(
            "on {} ",
            Color::Purple.bold().paint("mise healthy")
        ));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_folder_healthy() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_dir = dir.path().join(".mise");

        std::fs::create_dir_all(config_dir)?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
            })
            .cmd(
                "mise doctor",
                Some(CommandOutput {
                    stdout: String::default(),
                    stderr: String::default(),
                }),
            );

        let expected = Some(format!(
            "on {} ",
            Color::Purple.bold().paint("mise healthy")
        ));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }

    #[test]
    fn folder_with_mise_config_file_unhealthy() -> io::Result<()> {
        let dir = tempfile::tempdir()?;
        let config_path = dir.path().join(".mise.toml");

        std::fs::File::create(config_path)?.sync_all()?;

        let renderer = ModuleRenderer::new("mise")
            .path(dir.path())
            .config(toml::toml! {
                [mise]
                disabled = false
            })
            .cmd("mise doctor", None);

        let expected = Some(format!(
            "on {} ",
            Color::Purple.bold().paint("mise unhealthy")
        ));
        assert_eq!(expected, renderer.collect());

        dir.close()
    }
}
