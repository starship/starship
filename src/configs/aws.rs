use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;
use toml::Value;

#[derive(Clone, Debug, PartialEq)]
pub enum AwsItems {
    All,
    Region,
    Profile,
}

impl std::fmt::Display for AwsItems {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, ModuleConfig)]
pub struct AwsConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub profile: SegmentConfig<'a>,
    pub region: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
    pub displayed_items: AwsItems,
}

impl<'a> RootModuleConfig<'a> for AwsConfig<'a> {
    fn new() -> Self {
        AwsConfig {
            symbol: SegmentConfig::new("☁️  "),
            profile: SegmentConfig::default(),
            region: SegmentConfig::default(),
            style: Color::Yellow.bold(),
            disabled: false,
            displayed_items: AwsItems::All,
        }
    }
}

impl<'a> ModuleConfig<'a> for AwsItems {
    fn from_config(config: &Value) -> Option<AwsItems> {
        match config {
            Value::String(s) if s == "all" => Some(AwsItems::All),
            Value::String(s) if s == "region" => Some(AwsItems::Region),
            Value::String(s) if s == "profile" => Some(AwsItems::Profile),
            _ => None,
        }
    }
}
