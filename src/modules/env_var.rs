use super::{Context, Module};

use crate::config::RootModuleConfig;
use crate::configs::env_var::EnvVarConfig;
use crate::formatter::StringFormatter;
use crate::segment::Segment;


/// A utility module to display multiple env_variable modules
pub fn env_var_displayer<'a>(modules: Vec<Module>, context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("env_var_displayer");

    let module_segments = modules
        .into_iter()
        .flat_map(|module| module.segments)
        .collect::<Vec<Segment>>();
    module.set_segments(module_segments);
    Some(module)
}
/// Creates a module with the value of the chosen environment variable
///
/// Will display the environment variable's value if all of the following criteria are met:
///     - env_var.disabled is absent or false
///     - env_var.variable is defined
///     - a variable named as the value of env_var.variable is defined
pub fn module<'a>(
    variable_name: &str,
    context: &'a Context,
    config_value: Option<&toml::Value>,
) -> Option<Module<'a>> {
    let mut module = context.new_module(&["env_var", variable_name].join("."));
    let config = EnvVarConfig::load(config_value.expect(
        "modules::env_var::module should only be called after ensuring that the module exists",
    ));

    if config.disabled {
        return None;
    };

    let env_value = get_env_value(context, config.variable?, config.default)?;
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
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `env_var`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_env_value(context: &Context, name: &str, default: Option<&str>) -> Option<String> {
    match context.get_env(name) {
        Some(value) => Some(value),
        None => default.map(|value| value.to_owned()),
    }
}

#[cfg(test)]
mod test {
    use crate::test::ModuleRenderer;
    use ansi_term::{Color, Style};

    const TEST_VAR_VALUE: &str = "astronauts";

    #[test]
    fn defined_variable() {
        let actual = ModuleRenderer::new("env_var")
            .config(toml::toml! {
                [env_var]
                variable = "TEST_VAR"
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
                [env_var]
                variable = "TEST_VAR"
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn default_has_no_effect() {
        let actual = ModuleRenderer::new("env_var")
            .config(toml::toml! {
                [env_var]
                variable = "TEST_VAR"
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
                [env_var]
                variable = "UNDEFINED_TEST_VAR"
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
                [env_var]
                variable = "TEST_VAR"
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
                [env_var]
                variable = "TEST_VAR"
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
                [env_var]
                variable = "TEST_VAR"
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

    fn style() -> Style {
        // default style
        Color::Black.bold().dimmed()
    }
}
