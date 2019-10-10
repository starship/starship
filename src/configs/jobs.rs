use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JobsConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub threshold: i64,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for JobsConfig<'a> {
    fn new() -> Self {
        JobsConfig {
            symbol: SegmentConfig::new("✦"),
            threshold: 1,
            style: Color::Blue.bold(),
            disabled: false,
        }
    }
}
