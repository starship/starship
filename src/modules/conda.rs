use std::env;

use super::{Context, Module};

use crate::config::RootModuleConfig;
use crate::configs::conda::CondaConfig;

/// Creates a module with the current Conda environment
///
/// Will display the Conda environment iff `$CONDA_DEFAULT_ENV` is set.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    // Reference implementation: https://github.com/denysdovhan/spaceship-prompt/blob/master/sections/conda.zsh
    let conda_env = env::var("CONDA_DEFAULT_ENV").ok()?;
    if conda_env.is_empty() {
        return None;
    }

    let mut module = context.new_module("conda");
    let config = CondaConfig::try_load(module.config);

    module.set_style(config.style);

    module.create_segment("symbol", &config.symbol);
    module.create_segment("environment", &config.environment.with_value(&conda_env));

    Some(module)
}
