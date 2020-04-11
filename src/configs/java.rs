use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JavaConfig<'a> {
    pub disabled: bool,
    pub format: &'a str,
}

impl<'a> RootModuleConfig<'a> for JavaConfig<'a> {
    fn new() -> Self {
        JavaConfig {
            format: "via [☕ $version](red dimmed) ",
            disabled: false,
        }
    }
}
