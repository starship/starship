use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct DenoConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for DenoConfig<'a> {
    fn default() -> Self {
        DenoConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "🦕  ",
            style: "green bold",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["mod.ts", "deps.ts", "mod.js", "deps.js"],
            detect_folders: vec![],
        }
    }
}
