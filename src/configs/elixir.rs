use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ElixirConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> RootModuleConfig<'a> for ElixirConfig<'a> {
    fn new() -> Self {
        ElixirConfig {
            format: "via [$symbol($version \\(OTP $otp_version\\) )]($style)",
            symbol: "💧 ",
            style: "bold purple",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["mix.exs"],
            detect_folders: vec![],
        }
    }
}
