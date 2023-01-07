use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GcloudConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub region_aliases: HashMap<String, &'a str>,
    pub project_aliases: HashMap<String, &'a str>,
}

impl<'a> Default for GcloudConfig<'a> {
    fn default() -> Self {
        GcloudConfig {
            format: "on [$symbol$account(@$domain)(\\($region\\))]($style) ",
            symbol: "☁️  ",
            style: "bold blue",
            disabled: false,
            region_aliases: HashMap::new(),
            project_aliases: HashMap::new(),
        }
    }
}
