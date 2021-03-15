use crate::config::ModuleConfig;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct HostnameConfig<'a> {
    pub ssh_only: bool,
    pub trim_at: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for HostnameConfig<'a> {
    fn default() -> Self {
        HostnameConfig {
            ssh_only: true,
            trim_at: ".",
            format: "[$hostname]($style) in ",
            style: "green dimmed bold",
            disabled: false,
        }
    }
}
