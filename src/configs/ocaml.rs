use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct OCamlConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for OCamlConfig<'a> {
    fn new() -> Self {
        OCamlConfig {
            format: "via [$symbol$version]($style) ",
            symbol: "🐫 ",
            style: "bold yellow",
            disabled: false,
        }
    }
}
