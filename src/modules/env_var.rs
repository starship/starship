use ansi_term::Color;
use std::env;

use super::{Context, Module};

/// Creates a module with the value of the chosen environment variable
///
/// Will display the environment variable's value if all of the following criteria are met:
///     - env_var.disabled is absent or false
///     - env_var.variable is defined
///     - a variable named as the value of env_var.variable is defined
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("env_var");
    let module_style = module
        .config_value_style("style")
        .unwrap_or_else(|| Color::Black.bold().dimmed());

    let env_name = module.config_value_str("variable")?;

    let default_value = module.config_value_str("default");

    let env_value = get_env_value(env_name, default_value)?;

    let prefix = module.config_value_str("prefix").unwrap_or("").to_owned();
    let suffix = module.config_value_str("suffix").unwrap_or("").to_owned();

    module.set_style(module_style);
    module.get_prefix().set_value("with ");
    module.new_segment_if_config_exists("symbol");
    module.new_segment("env_var", &format!("{}{}{}", prefix, env_value, suffix));

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
