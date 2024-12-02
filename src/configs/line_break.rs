use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Default)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct LineBreakConfig {
    pub disabled: bool,
    pub break_below_width: Option<usize>,
}
