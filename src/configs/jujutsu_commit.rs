use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JujutsuCommitConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub prefix_style: &'a str,
    pub format: &'a str,
    pub commit_hash_length: usize,
    pub disabled: bool,
}

impl Default for JujutsuCommitConfig<'_> {
    fn default() -> Self {
        Self {
            disabled: true,
            format: "[$symbol[$commit_prefix]($prefix_style)$commit_suffix]($style) ",
            prefix_style: "blue",
            style: "",
            symbol: "",
            commit_hash_length: 7,
        }
    }
}
