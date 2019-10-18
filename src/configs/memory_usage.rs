use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct MemoryUsageConfig<'a> {
    pub show_percentage: bool,
    pub show_swap: bool,
    pub threshold: i64,
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for MemoryUsageConfig<'a> {
    fn new() -> Self {
        MemoryUsageConfig {
            show_percentage: false,
            show_swap: false,
            threshold: 75_i64,
            symbol: SegmentConfig::new("🐏 "),
            style: Color::White.dimmed(),
            disabled: true,
        }
    }
}
