use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct HostnameConfig<'a> {
    pub format: &'a str,
    pub ssh_only: bool,
    pub trim_at: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for HostnameConfig<'a> {
    fn new() -> Self {
        HostnameConfig {
            format: "on ${host?style=green bold dimmed} ",
            ssh_only: true,
            trim_at: ".",
            disabled: false,
        }
    }
}
