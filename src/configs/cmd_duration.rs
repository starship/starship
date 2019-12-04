use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CmdDurationConfig<'a> {
    pub min_time: i64,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for CmdDurationConfig<'a> {
    fn new() -> Self {
        CmdDurationConfig {
            min_time: 2,
            format: "${styled?value=took &style=yellow bold}${duration?style=yellow bold} ",
            disabled: false,
        }
    }
}
