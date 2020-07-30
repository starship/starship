use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct DockerConfig<'a> {
    pub symbol: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for DockerConfig<'a> {
    fn new() -> Self {
        DockerConfig {
            format: "[$symbol]($style) ",
            symbol: "🐳 ",
            style: "blue bold",
            disabled: false,
        }
    }
}
