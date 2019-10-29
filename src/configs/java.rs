use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JavaConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for JavaConfig<'a> {
    fn new() -> Self {
        JavaConfig {
            symbol: SegmentConfig::new("☕ "),
            style: Color::Red.dimmed(),
            disabled: false,
        }
    }
}
