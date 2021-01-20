use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct NimConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for NimConfig<'a> {
    fn new() -> Self {
        NimConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "👑 ",
            style: "yellow bold",
            disabled: false,
        }
    }
}
