use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct AzureConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for AzureConfig<'a> {
    fn new() -> Self {
        AzureConfig {
            format: "on ☁️ [$subscription](blue bold) ",
            disabled: false,
        }
    }
}
