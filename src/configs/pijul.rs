use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars:JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct PijulConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub truncation_length: i64,
    pub truncation_symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for PijulConfig<'a> {
    fn default() -> Self {
        PijulConfig {
            symbol: " ",
            style: "bold purple",
            format: "on [$symbol$channel]($style) ",
            truncation_length: std::i64::MAX,
            truncation_symbol: "…",
            disabled: true,
        }
    }
}
