use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct NodejsYarnConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for NodejsYarnConfig<'a> {
    fn new() -> Self {
        NodejsYarnConfig {
            symbol: SegmentConfig::new("🐈 "),
            style: Color::Blue.bold(),
            disabled: true,
        }
    }
}
