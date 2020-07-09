use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PackageConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub display_private: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PackageConfig<'a> {
    fn new() -> Self {
        PackageConfig {
            format: "is [$symbol$version]($style) ",
            symbol: "📦 ",
            style: "208 bold",
            display_private: false,
            disabled: false,
        }
    }
}
