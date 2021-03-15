use crate::config::ModuleConfig;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ElmConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for ElmConfig<'a> {
    fn default() -> Self {
        ElmConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "🌳 ",
            style: "cyan bold",
            disabled: false,
            detect_extensions: vec!["elm"],
            detect_files: vec!["elm.json", "elm-package.json", ".elm-version"],
            detect_folders: vec!["elm-stuff"],
        }
    }
}
