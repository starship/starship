use crate::config::{ModuleConfig, RootModuleConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct TimeConfig<'a> {
    pub use_12hr: bool,
    pub format: Option<&'a str>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for TimeConfig<'a> {
    fn new() -> Self {
        TimeConfig {
            use_12hr: false,
            format: None,
            style: Color::Yellow.bold(),
            disabled: true,
        }
    }
}
