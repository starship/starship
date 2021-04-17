use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;
use std::collections::HashMap;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct KubernetesConfig<'a> {
    pub symbol: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub context_aliases: HashMap<String, &'a str>,
}

impl<'a> Default for KubernetesConfig<'a> {
    fn default() -> Self {
        KubernetesConfig {
            symbol: "☸ ",
            format: "[$symbol$context( \\($namespace\\))]($style) in ",
            style: "cyan bold",
            disabled: true,
            context_aliases: HashMap::new(),
        }
    }
}
