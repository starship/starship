use dirs::home_dir;

use super::{Context, Module, RootModuleConfig};

use crate::configs::docker_context::DockerContextConfig;
use crate::utils;

const DOCKER_CONFIG_FILE: &str = ".docker/config.json";

/// Creates a module with the currently active Docker context
///
/// Will display the Docker context if the following criteria are met:
///     - There is a file named `$HOME/.docker/config.json`
///     - The file is JSON and contains a field named `currentContext`
///     - The value of `currentContext` is not `default`
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module(DOCKER_CONFIG_FILE);
    let config: DockerContextConfig = DockerContextConfig::try_load(module.config);

    let config_path = home_dir()?.join(".docker/config.json");
    let json = utils::read_file(config_path).ok()?;
    let parsed_json = serde_json::from_str(&json).ok()?;

    match parsed_json {
        serde_json::Value::Object(root) => {
            let current_context = root.get("currentContext")?;
            match current_context {
                serde_json::Value::String(ctx) => {
                    module.set_style(config.style);
                    module.create_segment("symbol", &config.symbol);
                    module.create_segment("context", &config.context.with_value(&ctx));
                    Some(module)
                }
                _ => None,
            }
        }
        _ => None,
    }
}
