use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;
use std::collections::HashMap;

#[derive(Clone, ModuleConfig)]
pub struct KubernetesConfig<'a> {
    pub symbol: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub namespace_spaceholder: &'a str,
    pub context_aliases: HashMap<String, &'a str>,
}

impl<'a> RootModuleConfig<'a> for KubernetesConfig<'a> {
    fn new() -> Self {
        KubernetesConfig {
            symbol: "☸ ",
            format: "on [$symbol$context \\($namespace\\)]($style) ",
            style: "cyan bold",
            disabled: true,
            namespace_spaceholder: "none",
            context_aliases: HashMap::new(),
        }
    }
}
