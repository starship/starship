use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct AzureRMConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub subscription_aliases: HashMap<String, &'a str>,
}

impl<'a> Default for AzureRMConfig<'a> {
    fn default() -> Self {
        AzureRMConfig {
            format: "on [$symbol($subscription)]($style) ",
            symbol: "󰠅 ",
            style: "blue bold",
            disabled: false ,
            subscription_aliases: HashMap::new(),
        }
    }
}
