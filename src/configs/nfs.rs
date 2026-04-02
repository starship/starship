use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct NfsConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for NfsConfig<'a> {
    fn default() -> Self {
        NfsConfig {
            format: "[$symbol$hostname]($style) ",
            symbol: "☁︎ ",
            style: "cyan bold",
            disabled: false,
        }
    }
}
