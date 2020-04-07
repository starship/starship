use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct NixShellConfig<'a> {
    pub use_name: bool,
    pub impure_msg: SegmentConfig<'a>,
    pub pure_msg: SegmentConfig<'a>,
    pub style: Style,
    pub symbol: SegmentConfig<'a>,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for NixShellConfig<'a> {
    fn new() -> Self {
        NixShellConfig {
            use_name: false,
            impure_msg: SegmentConfig::new("impure"),
            pure_msg: SegmentConfig::new("pure"),
            style: Color::Blue.bold(),
            symbol: SegmentConfig::new("❄️  "),
            disabled: false,
        }
    }
}
