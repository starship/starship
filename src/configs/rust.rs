use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct RustConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for RustConfig<'a> {
    fn new() -> Self {
        RustConfig {
            format: "${styled?value=🦀 &style=red bold}${version?style=red bold} ",
            disabled: false,
        }
    }
}
