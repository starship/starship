use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct CrystalConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for CrystalConfig<'a> {
    fn new() -> Self {
        CrystalConfig {
            symbol: SegmentConfig::new("🔮 "),
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
