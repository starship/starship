use crate::config::{ModuleConfig, RootModuleConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct BatteryConfig<'a> {
    pub full_symbol: &'a str,
    pub charging_symbol: &'a str,
    pub discharging_symbol: &'a str,
    pub unknown_symbol: Option<&'a str>,
    pub empty_symbol: Option<&'a str>,
    pub display: Vec<BatteryDisplayConfig>,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for BatteryConfig<'a> {
    fn new() -> Self {
        BatteryConfig {
            full_symbol: "•",
            charging_symbol: "↑",
            discharging_symbol: "↓",
            unknown_symbol: None,
            empty_symbol: None,
            display: vec![BatteryDisplayConfig {
                threshold: 10,
                style: Color::Red.bold(),
            }],
            disabled: false,
        }
    }
}

#[derive(Clone, ModuleConfig)]
pub struct BatteryDisplayConfig {
    pub threshold: i64,
    pub style: Style,
}
