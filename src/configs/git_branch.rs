use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitBranchConfig<'a> {
    pub truncation_length: i64,
    pub truncation_symbol: &'a str,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitBranchConfig<'a> {
    fn new() -> Self {
        GitBranchConfig {
            truncation_length: std::i64::MAX,
            truncation_symbol: "…",
            format: "on [ $branch](bold purple) ",
            disabled: false,
        }
    }
}
