use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GoConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GoConfig<'a> {
    fn new() -> Self {
        GoConfig {
            format: "via ${styled?value=🐹 &style=cyan bold}${version?style=cyan bold}",
            disabled: false,
        }
    }
}
