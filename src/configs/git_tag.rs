use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct GitTagConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for GitTagConfig<'a> {
    fn default() -> Self {
        GitTagConfig {
            format: "[\\($symbol$tag\\)]($style) ",
            symbol: "🏷 ",
            style: "yellow bold",
            disabled: false,
        }
    }
}
