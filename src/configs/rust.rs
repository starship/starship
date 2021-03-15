use crate::config::ModuleConfig;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct RustConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for RustConfig<'a> {
    fn default() -> Self {
        RustConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "🦀 ",
            style: "bold red",
            disabled: false,
            detect_extensions: vec!["rs"],
            detect_files: vec!["Cargo.toml"],
            detect_folders: vec![],
        }
    }
}
