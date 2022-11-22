use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct AzureConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for AzureConfig<'a> {
    fn default() -> Self {
        AzureConfig {
            format: "on [$symbol($subscription)]($style) ",
            symbol: "ﴃ ",
            style: "blue bold",
            disabled: true,
        }
    }
}
