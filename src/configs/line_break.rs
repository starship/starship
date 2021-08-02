use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct LineBreakConfig {
    pub disabled: bool,
}

impl<'a> Default for LineBreakConfig {
    fn default() -> Self {
        LineBreakConfig { disabled: false }
    }
}
