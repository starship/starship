use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ErlangConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for ErlangConfig<'a> {
    fn new() -> Self {
        ErlangConfig {
            format: "via [$symbol$version]($style) ",
            symbol: " ",
            style: "bold red",
            disabled: false,
        }
    }
}
