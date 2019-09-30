use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct RustConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

/* This is what the macro adds.
impl<'a> ModuleConfig<'a> for RustConfig<'a> {
    fn load_config(&self, config: &'a toml::Value) -> Self {
        let mut new_module_config = self.clone();
        if let toml::Value::Table(config) = config {
            if let Some(config_str) = config.get("symbol") {
                new_module_config.symbol = new_module_config.symbol.load_config(config_str);
            }
            if let Some(config_str) = config.get("disabled") {
                new_module_config.disabled = new_module_config.disabled.load_config(config_str);
            }
            if let Some(config_str) = config.get("style") {
                new_module_config.style = new_module_config.style.load_config(config_str);
            }
        }
        new_module_config
    }
    fn from_config(config: &'a toml::Value) -> Option<Self> {
        let config = config.as_table()?;
        Some(RustConfig {
            symbol: <&'a str>::from_config(config.get("symbol")?)?,
            style: <Style>::from_config(config.get("style")?)?,
            disabled: <bool>::from_config(config.get("disabled")?)?,
        })
    }
}
*/

impl<'a> RootModuleConfig<'a> for RustConfig<'a> {
    fn new() -> Self {
        RustConfig {
            symbol: SegmentConfig {
                value: "🦀 ",
                style: None,
            },
            version: SegmentConfig {
                value: "",
                style: None,
            },
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
