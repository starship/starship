use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
#[derive(Debug)]
pub struct SeparatorConfig {
    pub symbol: String,
    pub color: String,
    pub disabled: bool,
}

impl Default for SeparatorConfig {
    fn default() -> Self {
        SeparatorConfig {
            symbol: "▄".to_string(),
            color: "blue".to_string(),
            disabled: false,
        }
    }
}
