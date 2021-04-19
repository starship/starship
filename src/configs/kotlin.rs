use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct KotlinConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub kotlin_binary: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for KotlinConfig<'a> {
    fn default() -> Self {
        KotlinConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "🅺 ",
            style: "bold blue",
            kotlin_binary: "kotlin",
            disabled: false,
            detect_extensions: vec!["kt", "kts"],
            detect_files: vec![],
            detect_folders: vec![],
        }
    }
}
