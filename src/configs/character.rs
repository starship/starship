use crate::config::{ModuleConfig, RootModuleConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CharacterConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub error_symbol: &'a str,
    pub vicmd_symbol: &'a str,
    pub use_symbol_for_status: bool,
    pub style_success: Style,
    pub style_failure: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for CharacterConfig<'a> {
    fn new() -> Self {
        CharacterConfig {
            format: "${prompt_symbol} ",
            symbol: "❯",
            error_symbol: "✖",
            vicmd_symbol: "❮",
            use_symbol_for_status: false,
            style_success: Color::Green.bold(),
            style_failure: Color::Red.bold(),
            disabled: false,
        }
    }
}
