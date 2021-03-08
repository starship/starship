use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct SwiftConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> RootModuleConfig<'a> for SwiftConfig<'a> {
    fn new() -> Self {
        SwiftConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "🐦 ",
            style: "bold 202",
            disabled: false,
            detect_extensions: vec!["swift"],
            detect_files: vec!["Package.swift"],
            detect_folders: vec![],
        }
    }
}
