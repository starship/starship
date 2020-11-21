use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitBranchConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub truncation_length: i64,
    pub truncation_symbol: &'a str,
    pub only_attached: bool,
    pub show_remote: bool,
    pub always_show_remote: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitBranchConfig<'a> {
    fn new() -> Self {
        GitBranchConfig {
            format: "on [$symbol$branch]($style)(:[$remote]($style)) ",
            symbol: " ",
            style: "bold purple",
            truncation_length: std::i64::MAX,
            truncation_symbol: "…",
            only_attached: false,
            always_show_remote: false,
            disabled: false,
        }
    }
}
