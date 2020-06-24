use std::env;

use super::{Context, Module, SegmentConfig};

use crate::config::RootModuleConfig;
use crate::configs::shlvl::ShLvlConfig;

const SHLVL_ENV_VAR: &str = "SHLVL";

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let shlvl = get_shlvl_value();

    match shlvl {
        Some(shlvl_num) => {
            let mut module = context.new_module("shlvl");
            let config: ShLvlConfig = ShLvlConfig::try_load(module.config);

            if config.threshold <= shlvl_num {
                module.set_style(config.style);

                module.get_prefix().set_value(config.prefix);
                module.get_suffix().set_value(config.suffix);

                if let Some(symbol) = config.symbol {
                    module.create_segment("symbol", &symbol);
                }

                module.create_segment("value", &SegmentConfig::new(&shlvl_num.to_string()));

                Some(module)
            } else {
                None
            }
        }
        None => None,
    }
}

fn get_shlvl_value() -> Option<i64> {
    match env::var_os(SHLVL_ENV_VAR) {
        Some(os_value) => match os_value.into_string() {
            Ok(str_value) => match str_value.parse::<i64>() {
                Ok(value) => Some(value),
                _ => None,
            },
            _ => None,
        },
        None => None,
    }
}
