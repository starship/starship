use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Default)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct LineBreakConfig {
    pub disabled: bool,
}
