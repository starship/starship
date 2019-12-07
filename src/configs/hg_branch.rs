use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct HgBranchConfig<'a> {
    pub format: &'a str,
    pub truncation_length: i64,
    pub truncation_symbol: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for HgBranchConfig<'a> {
    fn new() -> Self {
        HgBranchConfig {
            format: "on ${styled?value= &style=purple bold}${name?style=purple bold} ",
            truncation_length: std::i64::MAX,
            truncation_symbol: "…",
            disabled: true,
        }
    }
}
