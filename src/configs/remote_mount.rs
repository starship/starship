use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct RemoteMountConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub type_aliases: HashMap<String, &'a str>,
}

impl<'a> Default for RemoteMountConfig<'a> {
    fn default() -> Self {
        RemoteMountConfig {
            format: "[$symbol($user@)$hostname]($style) ",
            symbol: "☁︎ ",
            style: "cyan bold",
            disabled: false,
            type_aliases: HashMap::new(),
        }
    }
}
