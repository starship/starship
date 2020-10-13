use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CMakeConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for CMakeConfig<'a> {
    fn new() -> Self {
        CMakeConfig {
            format: "via [$symbol$version]($style) ",
            symbol: "喝 ",
            style: "bold blue",
            disabled: false,
        }
    }
}
