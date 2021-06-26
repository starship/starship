use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct GitStatsConfig {
    pub disabled: bool,
}

impl Default for GitStatsConfig {
    fn default() -> Self {
        GitStatsConfig { disabled: false }
    }
}
