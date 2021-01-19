use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JuliaConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for JuliaConfig<'a> {
    fn new() -> Self {
        JuliaConfig {
            format: "via [$symbol$version ]($style)",
            symbol: "ஃ ",
            style: "bold purple",
            disabled: false,
        }
    }
}
