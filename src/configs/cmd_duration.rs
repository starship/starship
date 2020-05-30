use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CmdDurationConfig<'a> {
    pub min_time: i64,
    pub format: &'a str,
    pub style: &'a str,
    pub show_milliseconds: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for CmdDurationConfig<'a> {
    fn new() -> Self {
        CmdDurationConfig {
            min_time: 2_000,
            format: "took [$duration]($style) ",
            show_milliseconds: false,
            style: "yellow bold",
            disabled: false,
        }
    }
}
