use std::env;

use super::{Context, Module, SegmentConfig};

use crate::config::RootModuleConfig;
use crate::configs::shlvl::ShLvlConfig;

const SHLVL_ENV_VAR: &str = "SHLVL";

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let shlvl = get_shlvl_value()?;

    let mut module = context.new_module("shlvl");
    let config: ShLvlConfig = ShLvlConfig::try_load(module.config);

    if config.disabled {
        return None;
    }

    if config.threshold <= shlvl {
        module.set_style(config.style);

        module.get_prefix().set_value(config.prefix);
        module.get_suffix().set_value(config.suffix);

        if let Some(symbol) = config.symbol {
            module.create_segment("symbol", &symbol);
        }

        module.create_segment("value", &SegmentConfig::new(&shlvl.to_string()));

        Some(module)
    } else {
        None
    }
}

fn get_shlvl_value() -> Option<i64> {
    env::var(SHLVL_ENV_VAR).ok()?.parse::<i64>().ok()
}
