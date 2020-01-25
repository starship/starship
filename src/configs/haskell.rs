use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct HaskellConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for HaskellConfig<'a> {
    fn new() -> Self {
        HaskellConfig {
            symbol: SegmentConfig::new("λ "),
            version: SegmentConfig::default(),
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
