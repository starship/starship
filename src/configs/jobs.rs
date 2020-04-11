use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JobsConfig<'a> {
    pub threshold: i64,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for JobsConfig<'a> {
    fn new() -> Self {
        JobsConfig {
            threshold: 1,
            format: "[✦$number](blue bold) ",
            disabled: false,
        }
    }
}
