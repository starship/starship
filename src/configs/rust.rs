use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct RustConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub toolchain: bool,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for RustConfig<'a> {
    fn new() -> Self {
        RustConfig {
            symbol: SegmentConfig::new("🦀 "),
            version: SegmentConfig::default(),
            toolchain: false,
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
