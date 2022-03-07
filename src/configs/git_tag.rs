use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct GitTagConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub separator: &'a str,
    pub disabled: bool,
}

impl<'a> Default for GitTagConfig<'a> {
    fn default() -> Self {
        GitTagConfig {
            format: "[\\($symbol$tags\\)]($style) ",
            symbol: "🏷 ",
            style: "yellow bold",
            separator: " ",
            disabled: false,
        }
    }
}
