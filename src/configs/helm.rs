use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct HelmConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for HelmConfig<'a> {
    fn default() -> Self {
        HelmConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "⎈ ",
            style: "bold white",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["helmfile.yaml", "Chart.yaml"],
            detect_folders: vec![],
        }
    }
}
