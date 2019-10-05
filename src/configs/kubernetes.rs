use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct KubernetesConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub context: SegmentConfig<'a>,
    pub namespace: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for KubernetesConfig<'a> {
    fn new() -> Self {
        KubernetesConfig {
            symbol: SegmentConfig {
                value: "☸ ",
                style: None,
            },
            context: SegmentConfig {
                value: "",
                style: None,
            },
            namespace: SegmentConfig {
                value: "",
                style: None,
            },
            style: Color::Cyan.bold(),
            disabled: true,
        }
    }
}
