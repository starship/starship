use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JavaConfig<'a> {
    pub disabled: bool,
    pub format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
}

impl<'a> RootModuleConfig<'a> for JavaConfig<'a> {
    fn new() -> Self {
        JavaConfig {
            format: "via [$symbol $version]($style) ",
            disabled: false,
            style: "red dimmed",
            symbol: "☕",
        }
    }
}
