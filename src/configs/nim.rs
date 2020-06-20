use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct NimConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for NimConfig<'a> {
    fn new() -> Self {
        NimConfig {
            symbol: SegmentConfig::new("👑 "),
            version: SegmentConfig::default(),
            style: Color::Yellow.bold(),
            disabled: false,
        }
    }
}
