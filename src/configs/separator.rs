use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
#[derive(Debug)]
pub struct SeperatorConfig {
    pub symbol: String,
    pub color: String,
    pub disabled: bool,
}

impl Default for SeperatorConfig {
    fn default() -> Self {
        SeperatorConfig {
            symbol: "▄".to_string(),
            color: "blue".to_string(),
            disabled: false,
        }
    }
}
