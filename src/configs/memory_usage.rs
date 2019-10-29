use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct MemoryConfig<'a> {
    pub show_percentage: bool,
    pub show_swap: bool,
    pub threshold: i64,
    pub symbol: SegmentConfig<'a>,
    pub seperator: SegmentConfig<'a>,
    pub ram: SegmentConfig<'a>,
    pub swap: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for MemoryConfig<'a> {
    fn new() -> Self {
        MemoryConfig {
            show_percentage: false,
            show_swap: true,
            threshold: 75,
            symbol: SegmentConfig::new("🐏 "),
            seperator: SegmentConfig::new(" | "),
            ram: SegmentConfig::default(),
            swap: SegmentConfig::default(),
            style: Color::White.bold().dimmed(),
            disabled: true,
        }
    }
}
