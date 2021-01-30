use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct VagrantConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for VagrantConfig<'a> {
    fn new() -> Self {
        VagrantConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "⍱ ",
            style: "cyan bold",
            disabled: false,
        }
    }
}
