use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JuliaConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for JuliaConfig<'a> {
    fn new() -> Self {
        JuliaConfig {
            format: "via [ஃ $version](purple bold) ",
            disabled: false,
        }
    }
}
