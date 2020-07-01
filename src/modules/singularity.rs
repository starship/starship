use std::env;

use super::{Context, Module, RootModuleConfig};

use crate::configs::singularity::SingularityConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Singularity image
///
/// Will display the Singularity image if `$SINGULARITY_NAME` is set.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let singularity_env = env::var("SINGULARITY_NAME").ok();
    singularity_env.as_ref()?;

    let mut module = context.new_module("singularity");
    let config: SingularityConfig = SingularityConfig::try_load(module.config);

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
                "env" => singularity_env.as_ref().map(Ok),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `sigularity`: \n{}", error);
            return None;
        }
    });

    Some(module)
}
