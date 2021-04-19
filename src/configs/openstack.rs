use crate::config::ModuleConfig;
use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct OspConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for OspConfig<'a> {
    fn default() -> Self {
        OspConfig {
            format: "on [$symbol$cloud(\\($project\\))]($style) ",
            symbol: "☁️  ",
            style: "bold yellow",
            disabled: false,
        }
    }
}
