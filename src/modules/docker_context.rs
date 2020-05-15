use dirs::home_dir;
use std::env;
use std::path::PathBuf;

use super::{Context, Module, RootModuleConfig};

use crate::configs::docker_context::DockerContextConfig;
use crate::formatter::StringFormatter;
use crate::utils;

/// Creates a module with the currently active Docker context
///
/// Will display the Docker context if the following criteria are met:
///     - There is a file named `$HOME/.docker/config.json`
///     - Or a file named `$DOCKER_CONFIG/config.json`
///     - The file is JSON and contains a field named `currentContext`
///     - The value of `currentContext` is not `default`
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let mut module = context.new_module("docker_context");
    let config: DockerContextConfig = DockerContextConfig::try_load(module.config);

    if config.only_with_files
        && !context
            .try_begin_scan()?
            .set_files(&["docker-compose.yml", "Dockerfile"])
            .is_match()
    {
        return None;
    }
    let docker_config = PathBuf::from(
        &env::var_os("DOCKER_CONFIG").unwrap_or(home_dir()?.join(".docker").into_os_string()),
    )
    .join("config.json");

    if !docker_config.exists() {
        return None;
    }

    let json = utils::read_file(docker_config).ok()?;
    let parsed_json = serde_json::from_str(&json).ok()?;

    match parsed_json {
        serde_json::Value::Object(root) => {
            let current_context = root.get("currentContext")?;
            match current_context {
                serde_json::Value::String(ctx) => {
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
                                "context" => Some(Ok(ctx)),
                                _ => None,
                            })
                            .parse(None)
                    });

                    module.set_segments(match parsed {
                        Ok(segments) => segments,
                        Err(error) => {
                            log::warn!("Error in module `docker_context`:\n{}", error);
                            return None;
                        }
                    });

                    module.get_prefix().set_value("");
                    module.get_suffix().set_value("");

                    Some(module)
                }
                _ => None,
            }
        }
        _ => None,
    }
}
