use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct VirtualEnvConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for VirtualEnvConfig<'a> {
    fn new() -> Self {
        VirtualEnvConfig {
            format: "[\\($virtualenv\\)](yellow bold) ",
            disabled: false,
        }
    }
}
