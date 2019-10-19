use crate::config::{ModuleConfig, RootModuleConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct UsernameConfig {
    pub style_root: Style,
    pub style_user: Style,
    pub show_always: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for UsernameConfig {
    fn new() -> Self {
        UsernameConfig {
            style_root: Color::Red.bold(),
            style_user: Color::Yellow.bold(),
            show_always: false,
            disabled: false,
        }
    }
}
