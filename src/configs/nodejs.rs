use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct NodejsConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for NodejsConfig<'a> {
    fn new() -> Self {
        NodejsConfig {
            format: "via [⬢ $version](green bold) ",
            disabled: false,
        }
    }
}
