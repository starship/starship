use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct ChefConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for ChefConfig<'a> {
    fn default() -> Self {
        ChefConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "🍳 ",
            style: "bold red",
            disabled: false,
            detect_files: vec![
                "Berksfile",
                "Berksfile.lock",
                ".kitchen.yml",
                "kitchen.yml",
                "metadata.rb",
                "metadata.json",
            ],
            detect_folders: vec![
                "cookbooks",
                "recipes",
            ],
        }
    }
}
