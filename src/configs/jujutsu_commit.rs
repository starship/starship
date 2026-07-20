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
    pub prefix_style: &'a str,
    pub suffix_style: &'a str,
    pub format: &'a str,
    pub commit_hash_length: usize,
    pub disabled: bool,
}

impl Default for JujutsuCommitConfig<'_> {
    fn default() -> Self {
        Self {
            disabled: true,
            format: "[$commit_prefix]($prefix_style)$commit_suffix($suffix_style) ",
            prefix_style: "blue",
            suffix_style: "",
            symbol: "",
            commit_hash_length: 7,
        }
    }
}
