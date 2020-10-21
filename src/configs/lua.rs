use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct LuaConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for LuaConfig<'a> {
    fn new() -> Self {
        LuaConfig {
            format: "via [$symbol$version]($style) ",
            symbol: "🌙 ",
            style: "bold blue",
            disabled: false,
        }
    }
}
