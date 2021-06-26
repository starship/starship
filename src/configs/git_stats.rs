use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct GitStatsConfig<'a> {
    pub added_style: &'a str,
    pub modified_style: &'a str,
    pub deleted_style: &'a str,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> Default for GitStatsConfig<'a> {
    fn default() -> Self {
        GitStatsConfig {
            added_style: "bold green",
            modified_style: "bold yellow",
            deleted_style: "bold red",
            format: "[+$added_lines]($added_style) [~$modified_lines]($modified_style) [-$deleted_lines]($deleted_style)",
            disabled: false,
        }
    }
}
