use crate::config::{ModuleConfig, RootModuleConfig};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct OspConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for OspConfig<'a> {
    fn new() -> Self {
        OspConfig {
            format: "on [$symbol$cloud(\\($project\\))]($style) ",
            symbol: "☁️  ",
            style: "bold yellow",
            disabled: false,
        }
    }
}
