use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct NodejsConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for NodejsConfig<'a> {
    fn new() -> Self {
        NodejsConfig {
            symbol: SegmentConfig::new("⬢ "),
            style: Color::Green.bold(),
            disabled: false,
        }
    }
}
