use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct DotnetConfig<'a> {
    pub format: &'a str,
    pub heuristic: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for DotnetConfig<'a> {
    fn new() -> Self {
        DotnetConfig {
            format: "${styled?value=•NET &style=blue bold}${version?style=blue bold} ",
            heuristic: true,
            disabled: false,
        }
    }
}
