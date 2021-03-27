use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct PackageConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub display_private: bool,
    pub disabled: bool,
}

impl<'a> Default for PackageConfig<'a> {
    fn default() -> Self {
        PackageConfig {
            format: "is [$symbol$version]($style) ",
            symbol: "📦 ",
            style: "208 bold",
            display_private: false,
            disabled: false,
        }
    }
}
