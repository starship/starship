use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct DLangConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for DLangConfig<'a> {
    fn default() -> Self {
        DLangConfig {
            format: "via [$symbol($version )]($style)",
            symbol: " ",
            style: "#b03931",
            disabled: false,
            detect_extensions: vec!["d", "di"],
            detect_files: vec!["dub.sdl", "dub.json"],
            detect_folders: vec![],
        }
    }
}
