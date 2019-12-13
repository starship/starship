use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct EmberjsConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for EmberjsConfig<'a> {
    fn new() -> Self {
        EmberjsConfig {
            symbol: SegmentConfig::new("🐹 "),
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
