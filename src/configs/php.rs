use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PhpConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PhpConfig<'a> {
    fn new() -> Self {
        PhpConfig {
            symbol: SegmentConfig::new("🐘 "),
            style: Color::Fixed(147).bold(),
            disabled: false,
        }
    }
}
