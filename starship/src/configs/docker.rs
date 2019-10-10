use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct DockerConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
    pub show_versions: bool,
    pub show_compose: bool,
}

impl<'a> RootModuleConfig<'a> for DockerConfig<'a> {
    fn new() -> Self {
        DockerConfig {
            symbol: SegmentConfig {
                value: "🐳 ",
                style: None,
            },
            version: SegmentConfig {
                value: "",
                style: None,
            },
            style: Color::Blue.bold(),
            disabled: true,
            show_versions: true,
            show_compose: false
        }
    }
}
