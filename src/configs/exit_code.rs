use crate::config::{ModuleConfig, RootModuleConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ExitCodeConfig<'a> {
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for ExitCodeConfig<'a> {
    fn new() -> Self {
        ExitCodeConfig {
            prefix: "",
            suffix: " ",
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
