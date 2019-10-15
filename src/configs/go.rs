use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GoConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GoConfig<'a> {
    fn new() -> Self {
        GoConfig {
            symbol: SegmentConfig::new("🐹 "),
            version: SegmentConfig::default(),
            style: Color::Cyan.bold(),
            disabled: false,
        }
    }
}
