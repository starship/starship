use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CharacterConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub error_symbol: SegmentConfig<'a>,
    pub vicmd_symbol: SegmentConfig<'a>,
    pub use_symbol_for_status: bool,
    pub style_success: Style,
    pub style_failure: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for CharacterConfig<'a> {
    fn new() -> Self {
        CharacterConfig {
            symbol: SegmentConfig {
                value: "❯",
                style: None,
            },
            error_symbol: SegmentConfig {
                value: "✖",
                style: None,
            },
            vicmd_symbol: SegmentConfig {
                value: "❮",
                style: None,
            },
            use_symbol_for_status: false,
            style_success: Color::Green.bold(),
            style_failure: Color::Red.bold(),
            disabled: false,
        }
    }
}
