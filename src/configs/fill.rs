use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct FillConfig<'a> {
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for FillConfig<'a> {
    fn default() -> Self {
        FillConfig {
            style: "bold black",
            symbol: ".",
            disabled: false,
        }
    }
}
