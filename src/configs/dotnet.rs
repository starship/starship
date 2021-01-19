use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct DotnetConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub heuristic: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for DotnetConfig<'a> {
    fn new() -> Self {
        DotnetConfig {
            format: "[$symbol$version (🎯 $tfm )]($style)",
            symbol: "•NET ",
            style: "blue bold",
            heuristic: true,
            disabled: false,
        }
    }
}
