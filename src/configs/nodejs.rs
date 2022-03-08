use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct NodejsConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub not_capable_style: &'a str,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for NodejsConfig<'a> {
    fn default() -> Self {
        NodejsConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "⬢ ",
            style: "bold green",
            disabled: false,
            not_capable_style: "bold red",
            detect_extensions: vec!["js", "mjs", "cjs", "ts"],
            detect_files: vec!["package.json", ".node-version", ".nvmrc"],
            detect_folders: vec!["node_modules"],
        }
    }
}
