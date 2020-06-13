use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ErlangConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for ErlangConfig<'a> {
    fn new() -> Self {
        ErlangConfig {
            symbol: SegmentConfig::new("🖧 "),
            version: SegmentConfig::default(),
            prefix: "via ",
            suffix: " ",
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
