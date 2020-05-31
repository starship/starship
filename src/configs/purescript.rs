use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PureScriptConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PureScriptConfig<'a> {
    fn new() -> Self {
        PureScriptConfig {
            format: "via [$symbol$version]($style) ",
            symbol: "<=> ",
            style: "bold white",
            disabled: false,
        }
    }
}
