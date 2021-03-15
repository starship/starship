use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct CharacterConfig<'a> {
    pub format: &'a str,
    pub success_symbol: &'a str,
    pub cancel_symbol: &'a str,
    pub error_symbol: &'a str,
    pub vicmd_success_symbol: &'a str,
    pub vicmd_cancel_symbol: &'a str,
    pub vicmd_error_symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for CharacterConfig<'a> {
    fn default() -> Self {
        CharacterConfig {
            format: "$symbol ",
            success_symbol: "[❯](bold green)",
            cancel_symbol: "[❯](bold yellow)",
            error_symbol: "[❯](bold red)",
            vicmd_success_symbol: "[❮](bold green)",
            vicmd_cancel_symbol: "[❮](bold yellow)",
            vicmd_error_symbol: "[❮](bold red)",
            disabled: false,
        }
    }
}
