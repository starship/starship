use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct GitBranchConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub truncation_length: i64,
    pub truncation_symbol: &'a str,
    pub only_attached: bool,
    pub always_show_remote: bool,
    pub ignore_branches: Vec<&'a str>,
    pub disabled: bool,
}

impl<'a> Default for GitBranchConfig<'a> {
    fn default() -> Self {
        GitBranchConfig {
            format: "on [$symbol$branch]($style)(:[$remote]($style)) ",
            symbol: " ",
            style: "bold purple",
            truncation_length: std::i64::MAX,
            truncation_symbol: "…",
            only_attached: false,
            always_show_remote: false,
            ignore_branches: vec![],
            disabled: false,
        }
    }
}
