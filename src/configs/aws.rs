use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};
use std::collections::HashMap;

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, PartialEq)]
pub enum AwsItems {
    All,
    Region,
    Profile,
}

#[derive(Clone, ModuleConfig)]
pub struct AwsConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub profile: SegmentConfig<'a>,
    pub region: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
    pub displayed_items: AwsItems,
    pub region_aliases: HashMap<String, &'a str>,
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
            region_aliases: HashMap::new(),
        }
    }
}

impl<'a> ModuleConfig<'a> for AwsItems {
    fn from_config(config: &toml::Value) -> Option<Self> {
        match config.as_str()? {
            "all" => Some(AwsItems::All),
            "region" => Some(AwsItems::Region),
            "profile" => Some(AwsItems::Profile),
            _ => None,
        }
    }
}
