use std::env;

use super::{Context, Module};

use crate::config::RootModuleConfig;
use crate::configs::env_var::EnvVarConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the value of the chosen environment variable
///
/// Will display the environment variable's value if all of the following criteria are met:
///     - env_var.disabled is absent or false
///     - env_var.variable is defined
///     - a variable named as the value of env_var.variable is defined
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("env_var");
    let config: EnvVarConfig = EnvVarConfig::try_load(module.config);

    let env_value = get_env_value(config.variable?, config.default)?;
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
                // This may result in multiple calls to `get_module_version` when a user have
                // multiple `$version` variables defined in `format`.
                "env_value" => Some(Ok(env_value.clone())),
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
    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    Some(module)
}

fn get_env_value(name: &str, default: Option<&str>) -> Option<String> {
    match env::var_os(name) {
        Some(os_value) => match os_value.into_string() {
            Ok(value) => Some(value),
            Err(_error) => None,
        },
        None => default.map(|value| value.to_owned()),
    }
}
