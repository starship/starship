use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CharacterConfig<'a> {
    pub success_format: &'a str,
    pub error_format: &'a str,
    pub vicmd_format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for CharacterConfig<'a> {
    fn new() -> Self {
        CharacterConfig {
            success_format: "[❯](bold green) ",
            error_format: "[❯](bold red) ",
            vicmd_format: "[❮](bold green) ",
            disabled: false,
        }
    }
}
