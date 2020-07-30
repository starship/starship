use crate::configs::docker::DockerConfig;

use super::{Context, Module, RootModuleConfig};
use crate::formatter::StringFormatter;

/// Creates a segment to show if there is a Dockerfile or docker-compose.yml in current directory
///
/// Will display the symbol if any of the following criteria are met:
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
    let config: DockerConfig = DockerConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

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
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `docker`: \n{}", error);
            return None;
        }
    });

    Some(module)
}
