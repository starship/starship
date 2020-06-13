use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;
use std::collections::HashMap;

#[derive(Clone, ModuleConfig)]
pub struct KubernetesConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub context: SegmentConfig<'a>,
    pub namespace: SegmentConfig<'a>,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
    pub context_aliases: HashMap<String, &'a str>,
}

impl<'a> RootModuleConfig<'a> for KubernetesConfig<'a> {
    fn new() -> Self {
        KubernetesConfig {
            symbol: SegmentConfig::new("☸ "),
            context: SegmentConfig::default(),
            namespace: SegmentConfig::default(),
            prefix: "on ",
            suffix: " ",
            style: Color::Cyan.bold(),
            disabled: true,
            context_aliases: HashMap::new(),
        }
    }
}
