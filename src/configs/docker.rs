use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct DockerConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for DockerConfig<'a> {
    fn new() -> Self {
        DockerConfig {
            symbol: SegmentConfig {
                value: "🐳 ",
                style: None,
            },
            style: Color::Blue.bold(),
            disabled: true,
        }
    }
}
