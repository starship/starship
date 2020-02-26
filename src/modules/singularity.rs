use std::env;

use super::{Context, Module, SegmentConfig};

use crate::config::RootModuleConfig;
use crate::configs::singularity::SingularityConfig;

/// Creates a module with the current Singularity image
///
/// Will display the Singularity image if `$SINGULARITY_NAME` is set.
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let singularity_env = env::var("SINGULARITY_NAME").unwrap_or_else(|_| "".into());
    if singularity_env.trim().is_empty() {
        return None;
    }

    let mut module = context.new_module("singularity");
    let config = SingularityConfig::try_load(module.config);

    module.get_prefix().set_value(config.label);
    module.set_style(config.style);
    module.create_segment("symbol", &config.symbol);

    let env_var_stacked = format!("{}{}{}", config.prefix, singularity_env, config.suffix);
    module.create_segment("singularity", &SegmentConfig::new(&env_var_stacked));

    Some(module)
}
