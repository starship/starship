use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ZigConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> RootModuleConfig<'a> for ZigConfig<'a> {
    fn new() -> Self {
        ZigConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "↯ ",
            style: "bold yellow",
            disabled: false,
            detect_extensions: vec!["zig"],
            detect_files: vec![],
            detect_folders: vec![],
        }
    }
}
