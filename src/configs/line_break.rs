use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize, Default)]
pub struct LineBreakConfig {
    pub disabled: bool,
}
