use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct NodejsNpmConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for NodejsNpmConfig<'a> {
    fn new() -> Self {
        NodejsNpmConfig {
            symbol: SegmentConfig::new("npm "),
            style: Color::Red.bold(),
            disabled: true,
        }
    }
}
