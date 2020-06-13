use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct RubyConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for RubyConfig<'a> {
    fn new() -> Self {
        RubyConfig {
            symbol: SegmentConfig::new("💎 "),
            prefix: "via ",
            suffix: " ",
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
