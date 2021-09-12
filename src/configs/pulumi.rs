use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct PulumiConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for PulumiConfig<'a> {
    fn default() -> Self {
        PulumiConfig {
            format: "via [$symbol$stack]($style) ",
            version_format: "v${raw}",
            symbol: "🚀 ",
            style: "bold 5",
            disabled: false,
        }
    }
}
