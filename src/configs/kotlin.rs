use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct KotlinConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub kotlin_binary: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for KotlinConfig<'a> {
    fn new() -> Self {
        KotlinConfig {
            format: "via [$symbol(v$version )]($style)",
            symbol: "🅺 ",
            style: "bold blue",
            kotlin_binary: "kotlin",
            disabled: false,
        }
    }
}
