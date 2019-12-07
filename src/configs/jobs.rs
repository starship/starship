use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JobsConfig<'a> {
    pub format: &'a str,
    pub threshold: i64,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for JobsConfig<'a> {
    fn new() -> Self {
        JobsConfig {
            format: "${styled?value=✦&style=blue bold}${number?style=blue bold} ",
            threshold: 1,
            disabled: false,
        }
    }
}
