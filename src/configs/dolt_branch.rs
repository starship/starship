use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct DoltBranchConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub truncation_length: i64,
    pub truncation_symbol: &'a str,
    pub detect_file: Vec<&'a str>,
    pub detect_folder: Vec<&'a str>,
    pub disabled: bool,
}

impl<'a> Default for DoltBranchConfig<'a> {
    fn default() -> Self {
        DoltBranchConfig {
            format: "on [$symbol$branch]($style)",
            symbol: " ",
            style: "bold purple",
            truncation_length: std::i64::MAX,
            truncation_symbol: "…",
            detect_file: Vec::default(),
            detect_folder: vec![".dolt"],
            disabled: false,
        }
    }
}
