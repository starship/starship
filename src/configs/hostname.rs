use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct HostnameConfig<'a> {
    pub ssh_only: bool,
    pub trim_at: &'a str,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for HostnameConfig<'a> {
    fn new() -> Self {
        HostnameConfig {
            ssh_only: true,
            trim_at: ".",
            format: "on [$hostname](green dimmed bold) ",
            disabled: false,
        }
    }
}
