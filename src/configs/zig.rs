use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ZigConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for ZigConfig<'a> {
    fn new() -> Self {
        ZigConfig {
            format: "via [$symbol$version]($style)",
            symbol: "↯ ",
            style: "bold yellow",
            disabled: false,
        }
    }
}
