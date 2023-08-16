use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub subscription_aliases: HashMap<String, &'a str>,
}

impl<'a> Default for AzureConfig<'a> {
    fn default() -> Self {
        AzureConfig {
            format: "on [$symbol($subscription)]($style) ",
            symbol: "󰠅 ",
            style: "blue bold",
            disabled: true,
            subscription_aliases: HashMap::new(),
        }
    }
}
