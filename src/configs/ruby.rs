use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct RubyConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for RubyConfig<'a> {
    fn new() -> Self {
        RubyConfig {
            format: "via [$symbol$version]($style) ",
            symbol: "💎 ",
            style: "bold red",
            disabled: false,
        }
    }
}
