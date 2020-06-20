use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PhpConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PhpConfig<'a> {
    fn new() -> Self {
        PhpConfig {
            symbol: "🐘 ",
            style: "147 bold",
            format: "via [$symbol$version]($style) ",
            disabled: false,
        }
    }
}
