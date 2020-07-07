use std::env;

use super::{Context, Module, RootModuleConfig};

use super::utils::directory::truncate;
use crate::configs::conda::CondaConfig;
use crate::formatter::StringFormatter;

/// Creates a module with the current Conda environment
///
/// Will display the Conda environment iff `$CONDA_DEFAULT_ENV` is set.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    // Reference implementation: https://github.com/denysdovhan/spaceship-prompt/blob/master/sections/conda.zsh
    let conda_env = env::var("CONDA_DEFAULT_ENV").unwrap_or_else(|_| "".into());
    if conda_env.trim().is_empty() {
        return None;
    }

    let mut module = context.new_module("conda");
    let config: CondaConfig = CondaConfig::try_load(module.config);

    let conda_env = truncate(conda_env, config.truncation_length);

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
                "environment" => Some(Ok(conda_env.as_str())),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `conda`:\n{}", error);
            return None;
        }
    });

    Some(module)
}
