use crate::config::{ModuleConfig, RootModuleConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct HostnameConfig<'a> {
    pub ssh_only: bool,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for HostnameConfig<'a> {
    fn new() -> Self {
        HostnameConfig {
            ssh_only: true,
            prefix: "",
            suffix: "",
            style: Color::Green.bold().dimmed(),
            disabled: false,
        }
    }
}
