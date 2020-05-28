use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct NodejsConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for NodejsConfig<'a> {
    fn new() -> Self {
        NodejsConfig {
            format: "via [$symbol$version]($style) ",
            symbol: "⬢ ",
            style: "bold green",
            disabled: false,
        }
    }
}
