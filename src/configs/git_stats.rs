use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct GitStatsConfig<'a> {
    pub a_style: &'a str,
    pub m_style: &'a str,
    pub d_style: &'a str,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> Default for GitStatsConfig<'a> {
    fn default() -> Self {
        GitStatsConfig {
            a_style: "bold green",
            m_style: "bold yellow",
            d_style: "bold red",
            format: "[+$added]($a_style) [~$modified]($m_style) [-$deleted]($d_style) ",
            disabled: true,
        }
    }
}
