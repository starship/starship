use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct NixShellConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub impure_msg: &'a str,
    pub pure_msg: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for NixShellConfig<'a> {
    fn new() -> Self {
        NixShellConfig {
            format: "via [$symbol$state( \\($name\\))]($style) ",
            symbol: "❄️  ",
            style: "bold blue",
            impure_msg: "impure",
            pure_msg: "pure",
            disabled: false,
        }
    }
}
