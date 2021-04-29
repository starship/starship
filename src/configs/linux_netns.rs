use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;
#[derive(Clone, ModuleConfig, Serialize)]
pub struct LinuxNetNsConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for LinuxNetNsConfig<'a> {
    fn default() -> Self {
        LinuxNetNsConfig {
            format: "[[$symbol]($style) $ns ]($style bold)",
            symbol: "ﯱ",
            style: "blue",
            disabled: false,
        }
    }
}
