use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct RConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for RConfig<'a> {
    fn default() -> Self {
        RConfig {
            format: "via [${symbol}${version}]($style) ",
            style: "blue bold",
            symbol: "R ",
            disabled: false,
        }
    }
}
