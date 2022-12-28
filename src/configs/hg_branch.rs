use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct HgBranchConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub truncation_length: i64,
    pub truncation_symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for HgBranchConfig<'a> {
    fn default() -> Self {
        HgBranchConfig {
            symbol: " ",
            style: "bold purple",
            format: "on [$symbol$branch:$topic]($style) ",
            truncation_length: std::i64::MAX,
            truncation_symbol: "…",
            disabled: true,
        }
    }
}
