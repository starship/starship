use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct OspConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl Default for OspConfig<'_> {
    fn default() -> Self {
        OspConfig {
            format: "on [$symbol$cloud(\\($project\\))]($style) ",
            symbol: "☁️  ",
            style: "bold yellow",
            disabled: false,
        }
    }
}
