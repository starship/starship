use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitBranchConfig<'a> {
    pub format: &'a str,
    pub truncation_length: i64,
    pub truncation_symbol: &'a str,
    pub branch_name: SegmentConfig<'a>,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitBranchConfig<'a> {
    fn new() -> Self {
        GitBranchConfig {
            format: "on ${styled?value= &style=purple bold}${name?style=purple bold} ",
            truncation_length: std::i64::MAX,
            truncation_symbol: "…",
            branch_name: SegmentConfig::default(),
            disabled: false,
        }
    }
}
