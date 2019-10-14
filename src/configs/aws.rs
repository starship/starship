use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct AwsConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub profile: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for AwsConfig<'a> {
    fn new() -> Self {
        AwsConfig {
            symbol: SegmentConfig::new("☁️  "),
            profile: SegmentConfig::default(),
            style: Color::Yellow.bold(),
            disabled: false,
        }
    }
}
