use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;
use toml::Value;

#[derive(Clone, ModuleConfig)]
pub struct KubernetesConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub context: SegmentConfig<'a>,
    pub namespace: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
    pub environments: Vec<KubernetesEnvironmentConfig<'a>>,
}

impl<'a> RootModuleConfig<'a> for KubernetesConfig<'a> {
    fn new() -> Self {
        KubernetesConfig {
            symbol: SegmentConfig::new("☸ "),
            context: SegmentConfig::default(),
            namespace: SegmentConfig::default(),
            style: Color::Cyan.bold(),
            disabled: true,
            environments: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct KubernetesEnvironmentConfig<'a> {
    pub name: &'a str,
    pub symbol: Option<SegmentConfig<'a>>,
    pub style: Option<Style>,
}

impl<'a> ModuleConfig<'a> for KubernetesEnvironmentConfig<'a> {
    fn from_config(config: &'a Value) -> Option<Self> {
        let table = config.as_table()?;
        Some(KubernetesEnvironmentConfig {
            name: table.get("name").and_then(|v| <&str>::from_config(v))?,
            symbol: table
                .get("symbol")
                .and_then(|v| SegmentConfig::from_config(v)),
            style: table.get("style").and_then(|v| Style::from_config(v)),
        })
    }
}
