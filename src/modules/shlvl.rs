use std::env;

use super::{Context, Module};

use crate::config::RootModuleConfig;
use crate::configs::shlvl::ShLvlConfig;
use crate::formatter::StringFormatter;

const SHLVL_ENV_VAR: &str = "SHLVL";

pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let shlvl = get_shlvl_value()?;

    let mut module = context.new_module("shlvl");
    let config: ShLvlConfig = ShLvlConfig::try_load(module.config);

    if config.disabled || shlvl < config.threshold {
        return None;
    }

    let shlvl_str = &shlvl.to_string();

    let parsed = StringFormatter::new(config.format).and_then(|formatter| {
        formatter
            .map_meta(|var, _| match var {
                "symbol" => Some(config.symbol),
                _ => None,
            })
            .map_style(|variable| match variable {
                "style" => Some(Ok(config.style)),
                _ => None,
            })
            .map(|variable| match variable {
                "shlvl" => Some(Ok(shlvl_str)),
                _ => None,
            })
            .parse(None)
    });

    module.set_segments(match parsed {
        Ok(segments) => segments,
        Err(error) => {
            log::warn!("Error in module `shlvl`:\n{}", error);
            return None;
        }
    });

    Some(module)
}

fn get_shlvl_value() -> Option<i64> {
    env::var(SHLVL_ENV_VAR).ok()?.parse::<i64>().ok()
}
