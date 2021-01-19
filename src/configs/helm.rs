use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct HelmConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for HelmConfig<'a> {
    fn new() -> Self {
        HelmConfig {
            format: "via [$symbol$version ]($style)",
            symbol: "⎈ ",
            style: "bold white",
            disabled: false,
        }
    }
}
