use std::env;

use super::{Context, Module};

use crate::config::RootModuleConfig;
use crate::configs::env_var::EnvVarConfig;

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

    module.set_style(config.style);
    module.get_prefix().set_value("with ");

    if let Some(symbol) = config.symbol {
        module.create_segment("symbol", &symbol);
    }

    // TODO: Use native prefix and suffix instead of stacking custom ones together with env_value.
    module.new_segment("env_var", &format!("{}{}{}", config.prefix, env_value, config.suffix));

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
