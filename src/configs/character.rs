use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct CharacterConfig<'a> {
    pub format: &'a str,
    pub success_symbol: &'a str,
    pub error_symbol: &'a str,
    pub vicmd_symbol: &'a str,
    pub root_symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for CharacterConfig<'a> {
    fn default() -> Self {
        const DEFAULT_SUCCESS_SYMBOL: &str = "[❯](bold green)";
        CharacterConfig {
            format: "$symbol ",
            success_symbol: DEFAULT_SUCCESS_SYMBOL,
            error_symbol: "[❯](bold red)",
            vicmd_symbol: "[❮](bold green)",
            root_symbol: DEFAULT_SUCCESS_SYMBOL,
            disabled: false,
        }
    }
}
