use crate::config::VecOr;
use serde::{Deserialize, Serialize};
#[cfg(feature = "battery")]
use starship_battery::State as BatteryState;

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
            full_symbol: "󰁹 ",
            charging_symbol: "󰂄 ",
            discharging_symbol: "󰂃 ",
            unknown_symbol: "󰁽 ",
            empty_symbol: "󰂎 ",
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
    #[cfg(feature = "battery")]
    #[serde(deserialize_with = "deserialize_battery_state")]
    pub when: VecOr<BatteryState>,
    #[cfg(not(feature = "battery"))]
    pub when: VecOr<&'a str>,
}

impl<'a> Default for BatteryDisplayConfig<'a> {
    fn default() -> Self {
        BatteryDisplayConfig {
            threshold: 10,
            style: "red bold",
            charging_symbol: None,
            discharging_symbol: None,
            #[cfg(feature = "battery")]
            when: VecOr(vec![
                BatteryState::Unknown,
                BatteryState::Charging,
                BatteryState::Discharging,
                BatteryState::Empty,
                BatteryState::Full,
            ]),
            #[cfg(not(feature = "battery"))]
            when: VecOr(vec!["unknown", "charging", "discharging", "empty", "full"]),
        }
    }
}

/// TODO: Remove once #3941 is fixed
#[cfg(feature = "battery")]
fn deserialize_battery_state<'de, D>(de: D) -> Result<VecOr<BatteryState>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let states = VecOr::<String>::deserialize(de)?;
    let mut out = VecOr(Vec::with_capacity(states.0.len()));

    for state in states.0 {
        let state = match state.as_str() {
            "unknown" => BatteryState::Unknown,
            "charging" => BatteryState::Charging,
            "discharging" => BatteryState::Discharging,
            "empty" => BatteryState::Empty,
            "full" => BatteryState::Full,
            _ => return Err(serde::de::Error::custom(format!("invalid battery state {state:?}, expected one of: unknown, charging, discharging, empty, full"))),
        };

        out.0.push(state);
    }
    Ok(out)
}
