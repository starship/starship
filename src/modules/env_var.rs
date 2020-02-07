use std::env;

use super::{Context, Module, SegmentConfig};

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

    //rustc doesn't let you do an "if" and an "if let" in the same if statement
    // if this changes in the future this can become a lot cleaner
    let env_value = if config.trim_before_last != "" {
        if let Some(index) = env_value.rfind(config.trim_before_last) {
            env_value.split_at(index + 1).1
        } else {
            env_value.as_ref()
        }
    } else {
        env_value.as_ref()
    };

    module.set_style(config.style);
    module.get_prefix().set_value("with ");

    if let Some(symbol) = config.symbol {
        module.create_segment("symbol", &symbol);
    }

    // TODO: Use native prefix and suffix instead of stacking custom ones together with env_value.
    let env_var_stacked = format!("{}{}{}", config.prefix, env_value, config.suffix);
    module.create_segment("env_var", &SegmentConfig::new(&env_var_stacked));

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
