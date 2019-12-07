use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CondaConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for CondaConfig<'a> {
    fn new() -> Self {
        CondaConfig {
            format: "via ${styled?value=C &style=green bold}${env?style=green bold} ",
            disabled: false,
        }
    }
}
