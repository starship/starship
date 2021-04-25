use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct RLangConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for RLangConfig<'a> {
    fn default() -> Self {
        RLangConfig {
            format: "via [$symbol($version )]($style)",
            style: "blue bold",
            symbol: "📐 ",
            disabled: false,
            detect_extensions: vec!["R", "Rd", "Rmd", "Rproj", "Rsx"],
            detect_files: vec![".Rprofile"],
            detect_folders: vec![".Rproj.user"],
        }
    }
}
