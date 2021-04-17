use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct VLangConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for VLangConfig<'a> {
    fn default() -> Self {
        VLangConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "V ",
            style: "blue bold",
            disabled: false,
            detect_extensions: vec!["v"],
            detect_files: vec!["v.mod"],
            detect_folders: vec![],
        }
    }
}
