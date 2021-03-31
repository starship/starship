use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct BatteryConfig<'a> {
    pub full_symbol: &'a str,
    pub charging_symbol: &'a str,
    pub discharging_symbol: &'a str,
    pub unknown_symbol: &'a str,
    pub empty_symbol: &'a str,
    pub display: Vec<BatteryDisplayConfig<'a>>,
    pub disabled: bool,
    pub format: &'a str,
}

impl<'a> Default for BatteryConfig<'a> {
    fn default() -> Self {
        BatteryConfig {
            full_symbol: "",
            charging_symbol: "",
            discharging_symbol: "",
            unknown_symbol: "",
            empty_symbol: "",
            format: "[$symbol$percentage]($style) ",
            display: vec![BatteryDisplayConfig {
                threshold: 10,
                style: "red bold",
            }],
            disabled: false,
        }
    }
}

#[derive(Clone, ModuleConfig, Default, Serialize)]
pub struct BatteryDisplayConfig<'a> {
    pub threshold: i64,
    pub style: &'a str,
}
