use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct KubernetesConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for KubernetesConfig<'a> {
    fn new() -> Self {
        KubernetesConfig {
            format: "on ${styled?value=☸ &style=cyan bold}${context?style=cyan bold} ",
            disabled: true,
        }
    }
}
