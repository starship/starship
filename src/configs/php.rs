use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PhpConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PhpConfig<'a> {
    fn new() -> Self {
        PhpConfig {
            format: "${styled?value=🐘 &style=red bold} ",
            disabled: false,
        }
    }
}
