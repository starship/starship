use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct GoConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for GoConfig<'a> {
    fn default() -> Self {
        GoConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "🐹 ",
            style: "bold cyan",
            disabled: false,
            detect_extensions: vec!["go"],
            detect_files: vec![
                "go.mod",
                "go.sum",
                "glide.yaml",
                "Gopkg.yml",
                "Gopkg.lock",
                ".go-version",
            ],
            detect_folders: vec!["Godeps"],
        }
    }
}
