use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct DockerContextConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub only_with_files: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for DockerContextConfig<'a> {
    fn new() -> Self {
        DockerContextConfig {
            symbol: "🐳 ",
            style: "blue bold",
            format: "via [$symbol$context]($style) ",
            only_with_files: true,
            disabled: false,
        }
    }
}
