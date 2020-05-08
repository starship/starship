use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct HaskellConfig<'a> {
    pub disabled: bool,
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
}

impl<'a> RootModuleConfig<'a> for HaskellConfig<'a> {
    fn new() -> Self {
        HaskellConfig {
            disabled: false,
            format: "via [$symbol $version]($style) ",
            symbol: "λ",
            style: "bold red",
        }
    }
}
