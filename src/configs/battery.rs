use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct BatteryConfig<'a> {
    pub full_symbol: &'a str,
    pub charging_symbol: &'a str,
    pub discharging_symbol: &'a str,
    pub unknown_symbol: &'a str,
    pub empty_symbol: &'a str,
    #[serde(borrow)]
    pub display: Vec<BatteryDisplayConfig<'a>>,
    pub disabled: bool,
    pub format: &'a str,
}

impl<'a> Default for BatteryConfig<'a> {
    fn default() -> Self {
        BatteryConfig {
            full_symbol: " ",
            charging_symbol: " ",
            discharging_symbol: " ",
            unknown_symbol: " ",
            empty_symbol: " ",
            format: "[$symbol$percentage]($style) ",
            display: vec![BatteryDisplayConfig::default()],
            disabled: false,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct BatteryDisplayConfig<'a> {
    pub threshold: i64,
    pub style: &'a str,
    pub charging_symbol: Option<&'a str>,
    pub discharging_symbol: Option<&'a str>,
    pub only_on_discharge: bool,
}

impl<'a> Default for BatteryDisplayConfig<'a> {
    fn default() -> Self {
        BatteryDisplayConfig {
            threshold: 10,
            style: "red bold",
            charging_symbol: None,
            discharging_symbol: None,
            only_on_discharge: false,
        }
    }
}
