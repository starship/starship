use super::{Context, Module, ModuleConfig};

use super::utils::directory::truncate;
use crate::configs::conda::CondaConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Conda environment
///
/// Will display the Conda environment iff `$CONDA_DEFAULT_ENV` is set.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    // Reference implementation: https://github.com/denysdovhan/spaceship-prompt/blob/master/sections/conda.zsh
    let conda_env = context.get_env("CONDA_DEFAULT_ENV").unwrap_or_default();
    if conda_env.trim().is_empty() {
        return None;
    }

    let mut module = context.new_module("conda");
    let config: CondaConfig = CondaConfig::try_load(module.config);

    if config.ignore_base && conda_env == "base" {
        return None;
    }

    let conda_env = truncate(&conda_env, config.truncation_length).unwrap_or(conda_env);

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
                "environment" => Some(Ok(conda_env.as_str())),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `conda`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

#[cfg(test)]
mod tests {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::Color;

    #[test]
    fn not_in_env() {
        let actual = ModuleRenderer::new("conda").collect();

        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn ignore_base() {
        let actual = ModuleRenderer::new("conda")
            .env("CONDA_DEFAULT_ENV", "base")
            .config(toml::toml! {
                [conda]
                ignore_base = true
            })
            .collect();

        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn env_set() {
        let actual = ModuleRenderer::new("conda")
            .env("CONDA_DEFAULT_ENV", "astronauts")
            .collect();

        let expected = Some(format!(
            "via {} ",
            Color::Green.bold().paint("🅒 astronauts")
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn truncate() {
        let actual = ModuleRenderer::new("conda")
            .env("CONDA_DEFAULT_ENV", "/some/really/long/and/really/annoying/path/that/shouldnt/be/displayed/fully/conda/my_env")
            .collect();

        let expected = Some(format!("via {} ", Color::Green.bold().paint("🅒 my_env")));

        assert_eq!(expected, actual);
    }
}
