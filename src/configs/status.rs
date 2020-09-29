use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct StatusConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for StatusConfig<'a> {
    fn new() -> Self {
        StatusConfig {
            format: "[$symbol$status]($style) ",
            symbol: "✖",
            style: "bold red",
            disabled: true,
        }
    }
}
