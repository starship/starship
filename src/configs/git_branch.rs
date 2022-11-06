use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GitBranchConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub truncation_length: i64,
    pub truncation_symbol: &'a str,
    pub only_attached: bool,
    pub always_show_remote: bool,
    pub always_show_remote_branch: bool,
    pub ignore_branches: Vec<&'a str>,
    pub disabled: bool,
}

impl<'a> Default for GitBranchConfig<'a> {
    fn default() -> Self {
        GitBranchConfig {
            format: "on [$symbol$branch(:$remote_branch)]($style) ",
            symbol: " ",
            style: "bold purple",
            truncation_length: std::i64::MAX,
            truncation_symbol: "…",
            only_attached: false,
            always_show_remote: false,
            always_show_remote_branch: true,
            ignore_branches: vec![],
            disabled: false,
        }
    }
}
