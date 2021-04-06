use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct ToolboxConfig<'a> {
    pub symbol: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for ToolboxConfig<'a> {
    fn default() -> Self {
        ToolboxConfig {
            format: "[$symbol\\[$env\\]]($style) ",
            symbol: "",
            style: "bright-blue bold dimmed",
            disabled: false,
        }
    }
}
