use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct TerraformConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> RootModuleConfig<'a> for TerraformConfig<'a> {
    fn new() -> Self {
        TerraformConfig {
            format: "via [$symbol$workspace]($style) ",
            symbol: "💠 ",
            style: "bold 105",
            disabled: false,
            detect_extensions: vec!["tf", "hcl"],
            detect_files: vec![],
            detect_folders: vec![".terraform"],
        }
    }
}
