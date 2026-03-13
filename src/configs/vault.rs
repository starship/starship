use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]

pub struct VaultConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub show_within_days: u64,
    pub disabled: bool,
}

impl Default for VaultConfig<'_> {
    fn default() -> Self {
        Self {
            format: "[$symbol Token expires: $expire_time]($style)",
            symbol: "⚠️",
            style: "bold red",
            show_within_days: 7,
            disabled: false,
        }
    }
}