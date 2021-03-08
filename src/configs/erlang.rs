use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ErlangConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> RootModuleConfig<'a> for ErlangConfig<'a> {
    fn new() -> Self {
        ErlangConfig {
            format: "via [$symbol($version )]($style)",
            symbol: " ",
            style: "bold red",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["rebar.config", "erlang.mk"],
            detect_folders: vec![],
        }
    }
}
