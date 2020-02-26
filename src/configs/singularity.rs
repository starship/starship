use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct SingularityConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub label: &'a str,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for SingularityConfig<'a> {
    fn new() -> Self {
        SingularityConfig {
            symbol: SegmentConfig::default(),
            label: "",
            prefix: "[",
            suffix: "]",
            style: Color::Blue.bold().dimmed(),
            disabled: false,
        }
    }
}
