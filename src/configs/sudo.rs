use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct SudoConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub allow_windows: bool,
    pub disabled: bool,
}

impl<'a> Default for SudoConfig<'a> {
    fn default() -> Self {
        SudoConfig {
            format: "[as $symbol]($style)",
            symbol: "🧙 ",
            style: "bold blue",
            allow_windows: false,
            disabled: true,
        }
    }
}
