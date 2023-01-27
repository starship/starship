use super::{Context, Module};
use std::borrow::Cow;

use crate::config::ModuleConfig;
use crate::configs::env_var::EnvVarConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the value of the chosen environment variable
///
/// Will display the environment variable's value if all of the following criteria are met:
///     - `env_var.disabled` is absent or false
///     - `env_var.variable` is defined
///     - a variable named as the value of `env_var.variable` is defined
pub fn module<'a>(name: Option<&str>, context: &'a Context) -> Option<Module<'a>> {
    let toml_config = match name {
        Some(name) => context
            .config
            .get_config(&["env_var", name])
            .map(Cow::Borrowed),
        None => context
            .config
            .get_module_config("env_var")
            .and_then(filter_config)
            .map(Cow::Owned)
            .map(Some)?,
    };

    let mod_name = match name {
        Some(name) => format!("env_var.{name}"),
        None => "env_var".to_owned(),
    };

    let config = EnvVarConfig::try_load(toml_config.as_deref());
    // Note: Forward config if `Module` ends up needing `config`
    let mut module = Module::new(&mod_name, config.description, None);
    if config.disabled {
        return None;
    };

    let variable_name = config.variable.or(name)?;

    let env_value = context.get_env(variable_name);
    let env_value = env_value.as_deref().or(config.default)?;
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
                "env_value" => Some(Ok(env_value)),
                _ => None,
            })
            .parse(None, Some(context))
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

/// Filter `config` to only includes non-table values
/// This filters the top-level table to only include its specific configuation
fn filter_config(config: &toml::Value) -> Option<toml::Value> {
    let o = config
        .as_table()
        .map(|table| {
            table
                .iter()
                .filter(|(_key, val)| !val.is_table())
                .map(|(key, val)| (key.clone(), val.clone()))
                .collect::<toml::value::Table>()
        })
        .filter(|table| !table.is_empty())
        .map(toml::Value::Table);
    log::trace!("Filtered top-level env_var config: {o:?}");
    o
}

#[cfg(test)]
mod test {
    use crate::test::ModuleRenderer;
    use nu_ansi_term::{Color, Style};

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
        let actual = ModuleRenderer::new("env_var.TEST_VAR")
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
        let actual = ModuleRenderer::new("env_var.TEST_VAR")
            .config(toml::toml! {
                [env_var.TEST_VAR]
            })
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn default_has_no_effect() {
        let actual = ModuleRenderer::new("env_var.TEST_VAR")
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
        let actual = ModuleRenderer::new("env_var.UNDEFINED_TEST_VAR")
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
        let actual = ModuleRenderer::new("env_var.TEST_VAR")
            .config(toml::toml! {
                [env_var.TEST_VAR]
                format = "with [■ $env_value](black bold dimmed) "
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .collect();
        let expected = Some(format!(
            "with {} ",
            style().paint(format!("■ {TEST_VAR_VALUE}"))
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn prefix() {
        let actual = ModuleRenderer::new("env_var.TEST_VAR")
            .config(toml::toml! {
                [env_var.TEST_VAR]
                format = "with [_$env_value](black bold dimmed) "
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .collect();
        let expected = Some(format!(
            "with {} ",
            style().paint(format!("_{TEST_VAR_VALUE}"))
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn suffix() {
        let actual = ModuleRenderer::new("env_var.TEST_VAR")
            .config(toml::toml! {
                [env_var.TEST_VAR]
                format = "with [${env_value}_](black bold dimmed) "
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .collect();
        let expected = Some(format!(
            "with {} ",
            style().paint(format!("{TEST_VAR_VALUE}_"))
        ));

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_few() {
        let actual1 = ModuleRenderer::new("env_var.TEST_VAR")
            .config(toml::toml! {
                [env_var.TEST_VAR]
                [env_var.TEST_VAR2]
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .env("TEST_VAR2", TEST_VAR_VALUE)
            .collect();
        let actual2 = ModuleRenderer::new("env_var.TEST_VAR2")
            .config(toml::toml! {
                [env_var.TEST_VAR]
                [env_var.TEST_VAR2]
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .env("TEST_VAR2", TEST_VAR_VALUE)
            .collect();
        let expected = Some(format!("with {} ", style().paint(TEST_VAR_VALUE)));

        assert_eq!(expected, actual1);
        assert_eq!(expected, actual2);
    }

    #[test]
    fn mixed() {
        let cfg = toml::toml! {
            [env_var]
            variable = "TEST_VAR_OUTER"
            format = "$env_value"
            [env_var.TEST_VAR_INNER]
            format = "$env_value"
        };
        let actual_inner = ModuleRenderer::new("env_var.TEST_VAR_INNER")
            .config(cfg.clone())
            .env("TEST_VAR_OUTER", "outer")
            .env("TEST_VAR_INNER", "inner")
            .collect();

        assert_eq!(
            actual_inner.as_deref(),
            Some("inner"),
            "inner module should be rendered"
        );

        let actual_outer = ModuleRenderer::new("env_var")
            .config(cfg)
            .env("TEST_VAR_OUTER", "outer")
            .env("TEST_VAR_INNER", "inner")
            .collect();

        assert_eq!(
            actual_outer.as_deref(),
            Some("outer"),
            "outer module should be rendered"
        );
    }

    #[test]
    fn no_config() {
        let actual = ModuleRenderer::new("env_var.TEST_VAR")
            .env("TEST_VAR", TEST_VAR_VALUE)
            .collect();
        let expected = Some(format!("with {} ", style().paint(TEST_VAR_VALUE)));

        assert_eq!(expected, actual);
    }

    #[test]
    fn disabled_child() {
        let actual = ModuleRenderer::new("env_var.TEST_VAR")
            .config(toml::toml! {
                [env_var.TEST_VAR]
                disabled = true
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn disabled_root() {
        let actual = ModuleRenderer::new("env_var")
            .config(toml::toml! {
                [env_var]
                disabled = true
            })
            .env("TEST_VAR", TEST_VAR_VALUE)
            .collect();
        let expected = None;

        assert_eq!(expected, actual);
    }

    #[test]
    fn variable_override() {
        let actual = ModuleRenderer::new("env_var.TEST_VAR")
            .config(toml::toml! {
                [env_var.TEST_VAR]
                variable = "TEST_VAR2"
            })
            .env("TEST_VAR", "implicit name")
            .env("TEST_VAR2", "explicit name")
            .collect();
        let expected = Some(format!("with {} ", style().paint("explicit name")));

        assert_eq!(expected, actual);
    }

    fn style() -> Style {
        // default style
        Color::Black.bold().dimmed()
    }
}
