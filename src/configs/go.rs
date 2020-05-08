use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GoConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GoConfig<'a> {
    fn new() -> Self {
        GoConfig {
            format: "via [$symbol$version]($style) ",
            symbol: "🐹 ",
            style: "bold cyan",
            disabled: false,
        }
    }
}
