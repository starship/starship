use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JuliaConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> RootModuleConfig<'a> for JuliaConfig<'a> {
    fn new() -> Self {
        JuliaConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "ஃ ",
            style: "bold purple",
            disabled: false,
            detect_extensions: vec!["jl"],
            detect_files: vec!["Project.toml", "Manifest.toml"],
            detect_folders: vec![],
        }
    }
}
