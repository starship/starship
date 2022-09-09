use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct SpackConfig<'a> {
    pub truncation_length: usize,
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for SpackConfig<'a> {
    fn default() -> Self {
        SpackConfig {
            truncation_length: 1,
            format: "via [$symbol$environment]($style) ",
            symbol: "🅢 ",
            style: "blue bold",
            disabled: false,
        }
    }
}
