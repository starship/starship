use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct SpackConfig<'a> {
    pub truncation_length: usize,
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for SpackConfig<'a> {
    fn default() -> Self {
        SpackConfig {
            truncation_length: 1,
            format: "via [$symbol$environment]($style) ",
            symbol: "🅢 ",
            style: "blue bold",
            disabled: false,
        }
    }
}
