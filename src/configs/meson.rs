use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct MesonConfig<'a> {
    pub truncation_length: u32,
    pub truncation_symbol: &'a str,
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for MesonConfig<'a> {
    fn default() -> Self {
        MesonConfig {
            truncation_length: std::u32::MAX,
            truncation_symbol: "…",
            format: "via [$symbol$project]($style) ",
            symbol: "⬢ ",
            style: "blue bold",
            disabled: false,
        }
    }
}
