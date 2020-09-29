use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct SwiftConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for SwiftConfig<'a> {
    fn new() -> Self {
        SwiftConfig {
            format: "via [$symbol$version]($style) ",
            symbol: "🐦 ",
            style: "bold 202",
            disabled: false,
        }
    }
}
