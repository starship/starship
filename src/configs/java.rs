use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JavaConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for JavaConfig<'a> {
    fn new() -> Self {
        JavaConfig {
            format: "via ${styled?value=☕ &style=red dimmed}${version?style=red dimmed} ",
            disabled: false,
        }
    }
}
