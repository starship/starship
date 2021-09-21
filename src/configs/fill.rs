use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct FillConfig<'a> {
    pub style: &'a str,
    pub symbol: &'a str,
}

impl<'a> Default for FillConfig<'a> {
    fn default() -> Self {
        FillConfig {
            style: "bold black",
            symbol: ".",
        }
    }
}
