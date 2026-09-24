use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct BlankFillConfig {
    pub disabled: bool,
}

impl Default for BlankFillConfig {
    fn default() -> Self {
        Self { disabled: false }
    }
}
