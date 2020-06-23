use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PerlConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PerlConfig<'a> {
    fn new() -> Self {
        PerlConfig {
            symbol: SegmentConfig::new("🐪 "),
            style: Color::Fixed(149).bold(),
            disabled: false,
        }
    }
}
