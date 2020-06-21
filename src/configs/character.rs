use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CharacterConfig<'a> {
    pub format: &'a str,
    pub success_symbol: &'a str,
    pub error_symbol: &'a str,
    pub vicmd_symbol: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for CharacterConfig<'a> {
    fn new() -> Self {
        CharacterConfig {
            format: "$symbol ",
            success_symbol: "[❯](bold green)",
            error_symbol: "[❯](bold red)",
            vicmd_symbol: "[❮](bold green)",
            disabled: false,
        }
    }
}
