use crate::config::ModuleConfig;
use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct AzureConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for AzureConfig<'a> {
    fn default() -> Self {
        AzureConfig {
            format: "on [$symbol($subscription)]($style) ",
            symbol: "🅰 ",
            style: "blue bold",
            disabled: true,
        }
    }
}
