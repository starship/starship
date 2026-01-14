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
    pub format: &'a str,
    pub commit_hash_length: usize,
    pub disabled: bool,
    pub ignore_working_copy: bool,
}

impl Default for JujutsuCommitConfig<'_> {
    fn default() -> Self {
        Self {
            disabled: true,
            format: "[$symbol$commit]($style) ",
            style: "blue",
            symbol: "",
            commit_hash_length: 7,
            ignore_working_copy: true,
        }
    }
}
