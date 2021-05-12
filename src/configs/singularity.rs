use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct SingularityConfig<'a> {
    pub symbol: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for SingularityConfig<'a> {
    fn default() -> Self {
        SingularityConfig {
            format: "[$symbol\\[$env\\]]($style) ",
            symbol: "",
            style: "blue bold dimmed",
            disabled: false,
        }
    }
}
