use crate::config::ModuleConfig;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct VagrantConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for VagrantConfig<'a> {
    fn default() -> Self {
        VagrantConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "⍱ ",
            style: "cyan bold",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["Vagrantfile"],
            detect_folders: vec![],
        }
    }
}
