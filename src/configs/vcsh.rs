use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct VcshConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> Default for VcshConfig<'a> {
    fn default() -> Self {
        VcshConfig {
            symbol: "",
            style: "bold yellow",
            format: "vcsh [$symbol$repo]($style) ",
            disabled: false,
        }
    }
}
