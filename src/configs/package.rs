use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PackageConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PackageConfig<'a> {
    fn new() -> Self {
        PackageConfig {
            symbol: SegmentConfig::new("📦 "),
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
