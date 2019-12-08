use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct MemoryConfig<'a> {
    pub format: &'a str,
    pub threshold: i64,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for MemoryConfig<'a> {
    fn new() -> Self {
        MemoryConfig {
            format: "${ram?style=white bold dimmed}\
                     ${styled?value= | &style=white bold dimmed}\
                     ${swap?style=white bold dimmed} ",
            threshold: 75,
            disabled: true,
        }
    }
}
