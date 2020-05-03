use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ExitCodeConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for ExitCodeConfig<'a> {
    fn new() -> Self {
        ExitCodeConfig {
            format: "[✖$code](red bold) ",
            disabled: true,
        }
    }
}
