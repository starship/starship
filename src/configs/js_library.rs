use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};
use std::collections::HashMap;

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, PartialEq, Debug)]
pub struct JsLibraryConfig<'a> {
    pub map: HashMap<String, JsLibraryConfigItem<'a>>,
    pub prefix: &'a str,
    pub separator: &'a str,
    pub suffix: &'a str,
}

#[derive(Clone, PartialEq, Debug)]
pub struct JsLibraryConfigItem<'a> {
    pub dependency: &'a str,
    pub symbol: SegmentConfig<'a>,
    pub style: Option<Style>,
    pub disabled: bool,
}

impl<'a> ModuleConfig<'a> for JsLibraryConfigItem<'a> {
    // we can't automatically derive this because this method is applying
    // default values if none are explicitly specifier
    fn from_config(config: &'a toml::Value) -> Option<Self> {
        let config = config.as_table()?;
        Some(JsLibraryConfigItem {
            dependency: config.get("dependency").and_then(<&'a str>::from_config)?,
            symbol: config
                .get("symbol")
                .and_then(<SegmentConfig<'a>>::from_config)
                .unwrap_or_else(|| <SegmentConfig<'a>>::new("")),
            style: config.get("style").and_then(<Style>::from_config),
            disabled: config
                .get("disabled")
                .and_then(<bool>::from_config)
                .unwrap_or(false),
        })
    }

    fn load_config(&self, config: &'a toml::Value) -> Self {
        let mut new_module_config = self.clone();
        if let toml::Value::Table(config) = config {
            if let Some(config_str) = config.get("dependency") {
                new_module_config.dependency = new_module_config.dependency.load_config(config_str);
            }
            if let Some(config_str) = config.get("symbol") {
                new_module_config.symbol = new_module_config.symbol.load_config(config_str);
            }
            if let Some(config_str) = config.get("style") {
                new_module_config.style = new_module_config.style.load_config(config_str);
            }
            if let Some(config_str) = config.get("disabled") {
                new_module_config.disabled = new_module_config.disabled.load_config(config_str);
            }
        }
        new_module_config
    }
}

impl<'a> RootModuleConfig<'a> for JsLibraryConfig<'a> {
    fn new() -> Self {
        JsLibraryConfig {
            map: {
                let mut map = HashMap::new();
                map.insert(
                    "emberjs".to_string(),
                    JsLibraryConfigItem {
                        dependency: "ember-source",
                        symbol: SegmentConfig::new("🐹 "),
                        style: Some(Color::Red.bold()),
                        disabled: false,
                    },
                );
                map.insert(
                    "react".to_string(),
                    JsLibraryConfigItem {
                        dependency: "react",
                        symbol: SegmentConfig::new("⚛️ "),
                        style: Some(Color::Blue.bold()),
                        disabled: false,
                    },
                );
                map
            },
            prefix: "with ",
            separator: " and ",
            suffix: " ",
        }
    }
}
