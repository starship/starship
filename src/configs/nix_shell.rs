use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct NixShellConfig<'a> {
    pub format: &'a str,
    pub impure_msg: &'a str,
    pub pure_msg: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for NixShellConfig<'a> {
    fn new() -> Self {
        NixShellConfig {
            format:
                "via ${name?style=red bold}${impure_msg?style=red bold}${pure_msg?style=red bold} ",
            impure_msg: "impure",
            pure_msg: "pure",
            disabled: false,
        }
    }
}
