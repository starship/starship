use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CrystalConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for CrystalConfig<'a> {
    fn new() -> Self {
        CrystalConfig {
            format: "via [$symbol(v$version )]($style)",
            symbol: "🔮 ",
            style: "bold red",
            disabled: false,
        }
    }
}
