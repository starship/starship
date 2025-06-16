use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct FossilBranchConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub truncation_length: i64,
    pub truncation_symbol: &'a str,
    pub disabled: bool,
}

impl Default for FossilBranchConfig<'_> {
    fn default() -> Self {
        FossilBranchConfig {
            format: "on [$symbol$branch]($style) ",
            symbol: " ",
            style: "bold purple",
            truncation_length: i64::MAX,
            truncation_symbol: "…",
            disabled: true,
        }
    }
}
