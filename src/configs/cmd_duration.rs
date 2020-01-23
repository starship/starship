use crate::config::{ModuleConfig, RootModuleConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CmdDurationConfig<'a> {
    pub min_time: i64,
    pub prefix: &'a str,
    pub style: Style,
    pub show_milliseconds: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for CmdDurationConfig<'a> {
    fn new() -> Self {
        CmdDurationConfig {
            min_time: 2_000,
            prefix: "took ",
            show_milliseconds: false,
            style: Color::Yellow.bold(),
            disabled: false,
        }
    }
}
