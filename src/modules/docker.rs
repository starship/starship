use std::process::Command;

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

    if config.show_versions {
        if let Some(docker_version) = get_docker_version() {
            let version = format!("{}{}", "v", docker_version.trim());
            module.create_segment("version", &config.version.with_value(&version));

            if config.show_compose {
                if let Some(compose_version) = get_docker_compose_version() {
                    module.create_segment("prefix", &config.symbol.with_value(&" with v"));
                    module.create_segment(
                        "version",
                        &config.version.with_value(&compose_version.trim()),
                    );
                }
            }
        }
    };
    Some(module)
}

fn get_docker_version() -> Option<String> {
    match Command::new("docker")
        .arg("version")
        .arg("--format")
        .arg("{{.Server.Version}}")
        .output()
    {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}

fn get_docker_compose_version() -> Option<String> {
    match Command::new("docker-compose")
        .arg("version")
        .arg("--short")
        .output()
    {
        Ok(output) => Some(String::from_utf8(output.stdout).unwrap()),
        Err(_) => None,
    }
}
