use crate::config::RootModuleConfig;
use crate::configs::docker::DockerConfig;

use super::{Context, Module};

/// Creates a module with the current Docker and Docker Compose version
///
/// Will display the Docker and Docker Compose version if any of the following criteria are met:
///     - Current directory contains a `Dockerfile` file
///     - Current directory contains a `docker-compose.yml` file
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let docker_used = context
        .try_begin_scan()?
        .set_files(&["Dockerfile", "docker-compose.yml"])
        .is_match();

    if !docker_used {
        return None;
    }

    let mut module = context.new_module("docker");
    let config = DockerConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    module.set_style(config.style);
    module.create_segment("symbol", &config.symbol);

    Some(module)
}
