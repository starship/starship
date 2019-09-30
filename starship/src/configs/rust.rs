use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct RustConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for RustConfig<'a> {
    fn new() -> Self {
        RustConfig {
            symbol: SegmentConfig {
                value: "🦀 ",
                style: None,
            },
            version: SegmentConfig {
                value: "",
                style: None,
            },
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
