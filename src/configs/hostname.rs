use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct HostnameConfig<'a> {
    pub ssh_only: bool,
    pub container_only: bool,
    pub trim_at: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for HostnameConfig<'a> {
    fn default() -> Self {
        HostnameConfig {
            ssh_only: true,
            container_only: false,
            trim_at: ".",
            format: "[$hostname]($style) in ",
            style: "green dimmed bold",
            disabled: false,
        }
    }
}
