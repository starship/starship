use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct TerraformConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for TerraformConfig<'a> {
    fn new() -> Self {
        TerraformConfig {
            format: "via [$symbol$workspace]($style) ",
            symbol: "💠 ",
            style: "bold 105",
            disabled: false,
        }
    }
}
