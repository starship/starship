use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct ImbaConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for ImbaConfig<'a> {
    fn default() -> Self {
        ImbaConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "🐤 ",
            style: "yellow bold",
            disabled: false,
            detect_extensions: vec!["imba"],
            detect_files: vec![],
            detect_folders: vec![],
        }
    }
}
