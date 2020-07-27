use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PerlConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PerlConfig<'a> {
    fn new() -> Self {
        PerlConfig {
            symbol: "🐪 ",
            style: "149 bold",
            format: "via [$symbol$version]($style) ",
            disabled: false,
        }
    }
}
