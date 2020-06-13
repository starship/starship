use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct DotnetConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub heuristic: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for DotnetConfig<'a> {
    fn new() -> Self {
        DotnetConfig {
            symbol: SegmentConfig::new("•NET "),
            version: SegmentConfig::default(),
            prefix: "via ",
            suffix: " ",
            style: Color::Blue.bold(),
            heuristic: true,
            disabled: false,
        }
    }
}
