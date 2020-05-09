use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct StatusConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for StatusConfig<'a> {
    fn new() -> Self {
        StatusConfig {
            format: "[✖$status](red bold) ",
            disabled: true,
        }
    }
}
