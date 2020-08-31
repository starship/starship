use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitBranchConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub prefix: &'a str,
    pub truncation_length: i64,
    pub truncation_symbol: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitBranchConfig<'a> {
    fn new() -> Self {
        GitBranchConfig {
            format: "$prefix[$symbol$branch]($style) ",
            symbol: " ",
            style: "bold purple",
            prefix: "on ",
            truncation_length: std::i64::MAX,
            truncation_symbol: "…",
            disabled: false,
        }
    }
}
