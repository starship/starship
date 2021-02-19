use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CMakeConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> RootModuleConfig<'a> for CMakeConfig<'a> {
    fn new() -> Self {
        CMakeConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "喝 ",
            style: "bold blue",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["CMakeLists.txt", "CMakeCache.txt"],
            detect_folders: vec![],
        }
    }
}
