use crate::config::ModuleConfig;

use starship_module_config_derive::ModuleConfig;
use toml::Value;

#[derive(Clone, ModuleConfig)]
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
            display: vec![BatteryDisplayConfig::new()],
            disabled: false,
        }
    }
}

#[derive(Clone)]
pub struct BatteryDisplayConfig<'a> {
    pub threshold: i64,
    pub style: &'a str,
    pub symbol: &'a str,
}

impl<'a> RootModuleConfig<'a> for BatteryDisplayConfig<'a> {
    fn new() -> Self {
        BatteryDisplayConfig {
            threshold: 10,
            style: "red bold",
            symbol: "",
        }
    }
}

impl<'a> ModuleConfig<'a> for BatteryDisplayConfig<'a> {
    fn from_config(config: &'a Value) -> Option<Self> {
        let mut conf = BatteryDisplayConfig::new();

        for (key, val) in config.as_table()?.iter() {
            let s = key.to_string();
            match &*s {
                "threshold" => conf.threshold = val.as_integer()?,
                "style" => conf.style = val.as_str()?,
                "symbol" => conf.symbol = val.as_str()?,
                _ => (),
            }
        }
        Some(conf)
    }
}
