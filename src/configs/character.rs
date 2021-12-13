use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct CharacterConfig<'a> {
    pub format: &'a str,
    pub success_symbol: &'a str,
    pub error_symbol: &'a str,
    pub continuation_symbol: &'a str,
    pub vicmd_symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for CharacterConfig<'a> {
    fn default() -> Self {
        CharacterConfig {
            format: "$symbol ",
            success_symbol: "[❯](bold green)",
            error_symbol: "[❯](bold red)",
            continuation_symbol: "[❯](bold yellow)",
            vicmd_symbol: "[❮](bold green)",
            disabled: false,
        }
    }
}
