use crate::config::ModuleConfig;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PhpConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for PhpConfig<'a> {
    fn default() -> Self {
        PhpConfig {
            symbol: "🐘 ",
            style: "147 bold",
            format: "via [$symbol($version )]($style)",
            disabled: false,
            detect_extensions: vec!["php"],
            detect_files: vec!["composer.json", ".php-version"],
            detect_folders: vec![],
        }
    }
}
