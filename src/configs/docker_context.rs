use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct DockerContextConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub context: SegmentConfig<'a>,
    pub style: Style,
    pub only_with_files: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for DockerContextConfig<'a> {
    fn new() -> Self {
        DockerContextConfig {
            symbol: SegmentConfig::new("🐳 "),
            context: SegmentConfig::default(),
            style: Color::Blue.bold(),
            only_with_files: false,
            disabled: true,
        }
    }
}
