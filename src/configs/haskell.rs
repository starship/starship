use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct HaskellConfig<'a> {
    pub disabled: bool,
    pub format: &'a str,
}

impl<'a> RootModuleConfig<'a> for HaskellConfig<'a> {
    fn new() -> Self {
        HaskellConfig {
            disabled: false,
            format: "via [λ $version](red bold) ",
        }
    }
}
