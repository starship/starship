use super::{Context, Module};

use crate::config::RootModuleConfig;
use crate::configs::env_var::EnvVarConfig;
use crate::formatter::StringFormatter;

/// Creates env_var_module displayer which displays all configured environmental variables
pub fn module<'a>(name: &str, context: &'a Context) -> Option<Module<'a>> {
    let toml_config = context.config.get_env_var_module_config(name).expect(
        "modules::env_var::module should only be called after ensuring that the module exists",
    );
    let config: EnvVarConfig = EnvVarConfig::load(toml_config);

    if config.disabled {
        return None;
    };

    let mut module = Module::new(
        &format!("env_var.{}", name),
        config.description,
        Some(toml_config),
    );

    let variable_name = get_variable_name(vec!["env_var", name], &config);

    let env_value = get_env_value(context, variable_name?, config.default)?;
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
                "env_value" => Some(Ok(&env_value)),
                _ => None,
            })
            .parse(None, Some(context))
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `env_var.{}`:\n{}", name, error);
            return None;
        }
    });

    Some(module)
}

fn get_variable_name<'a>(
    module_config_path: Vec<&'a str>,
    config: &'a EnvVarConfig,
) -> Option<&'a str> {
    match config.variable {
        Some(v) => Some(v),
        None => {
            let last_element = module_config_path.last()?;
            Some(*last_element)
        }
    }
}

fn get_env_value(context: &Context, name: &str, default: Option<&str>) -> Option<String> {
    match context.get_env(name) {
        Some(value) => Some(value),
        None => default.map(std::borrow::ToOwned::to_owned),
    }
}

#[cfg(test)]
mod test {
    use crate::test::ModuleRenderer;
    use ansi_term::{Color, Style};

    const TEST_VAR_VALUE: &str = "astronauts";

    #[test]
    fn empty_config() {
        let actual = ModuleRenderer::new("env_var").collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn fallback_config() {
        let actual = ModuleRenderer::new("env_var")
            .config(toml::toml! {
                [env_var]
                variable="TEST_VAR"
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .collect();
        let expected = Some(format!("with {} ", style().paint(TEST_VAR_VALUE)));

        assert_eq!(expected, actual);
    }

    #[test]
    fn defined_variable() {
        let actual = ModuleRenderer::new("env_var")
            .config(toml::toml! {
                [env_var.TEST_VAR]
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .collect();
        let expected = Some(format!("with {} ", style().paint(TEST_VAR_VALUE)));

        assert_eq!(expected, actual);
    }

    #[test]
    fn undefined_variable() {
        let actual = ModuleRenderer::new("env_var")
            .config(toml::toml! {
                [env_var.TEST_VAR]
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn default_has_no_effect() {
        let actual = ModuleRenderer::new("env_var")
            .config(toml::toml! {
                [env_var.TEST_VAR]
                default = "N/A"
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .collect();
        let expected = Some(format!("with {} ", style().paint(TEST_VAR_VALUE)));

        assert_eq!(expected, actual);
    }

    #[test]
    fn default_takes_effect() {
        let actual = ModuleRenderer::new("env_var")
            .config(toml::toml! {
                [env_var.UNDEFINED_TEST_VAR]
                default = "N/A"
            })
            .collect();
        let expected = Some(format!("with {} ", style().paint("N/A")));

        assert_eq!(expected, actual);
    }

    #[test]
    fn symbol() {
        let actual = ModuleRenderer::new("env_var")
            .config(toml::toml! {
                [env_var.TEST_VAR]
                format = "with [■ $env_value](black bold dimmed) "
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .collect();
        let expected = Some(format!(
            "with {} ",
            style().paint(format!("■ {}", TEST_VAR_VALUE))
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn prefix() {
        let actual = ModuleRenderer::new("env_var")
            .config(toml::toml! {
                [env_var.TEST_VAR]
                format = "with [_$env_value](black bold dimmed) "
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .collect();
        let expected = Some(format!(
            "with {} ",
            style().paint(format!("_{}", TEST_VAR_VALUE))
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn suffix() {
        let actual = ModuleRenderer::new("env_var")
            .config(toml::toml! {
                [env_var.TEST_VAR]
                format = "with [${env_value}_](black bold dimmed) "
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .collect();
        let expected = Some(format!(
            "with {} ",
            style().paint(format!("{}_", TEST_VAR_VALUE))
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_few() {
        let actual = ModuleRenderer::new("env_var")
            .config(toml::toml! {
                [env_var.TEST_VAR]
                [env_var.TEST_VAR2]
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .env("TEST_VAR2", TEST_VAR_VALUE)
            .collect();
        let expected = Some(format!(
            "with {} with {} ",
            style().paint(TEST_VAR_VALUE),
            style().paint(TEST_VAR_VALUE)
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_single_with_multiple_defined() {
        let actual = ModuleRenderer::new("env_var.TEST_VAR")
            .config(toml::toml! {
                [env_var.TEST_VAR]
                [env_var.TEST_VAR2]
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .env("TEST_VAR2", TEST_VAR_VALUE)
            .collect();
        let expected = Some(format!(
            "with {} ",
            style().paint(TEST_VAR_VALUE),
        ));

        assert_eq!(expected, actual);
    }

    fn style() -> Style {
        // default style
        Color::Black.bold().dimmed()
    }
}
