use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct ContainerConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for ContainerConfig<'a> {
    fn default() -> Self {
        ContainerConfig {
            format: "[$symbol \\[$name\\]]($style) ",
            symbol: "⬢",
            style: "red bold dimmed",
            disabled: false,
        }
    }
}
