use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JjStatusConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub conflicted: &'a str,
    pub ahead: &'a str,
    pub behind: &'a str,
    pub diverged: &'a str,
    pub modified: &'a str,
    pub staged: &'a str,
    pub renamed: &'a str,
    pub deleted: &'a str,
    pub untracked: &'a str,
    pub conflicted_count: bool,
    pub ahead_behind: bool,
    pub disabled: bool,
}

impl Default for JjStatusConfig<'_> {
    fn default() -> Self {
        Self {
            format: "([\\[$all_status$ahead_behind\\]]($style) )",
            style: "red bold",
            conflicted: "=",
            ahead: "⇡",
            behind: "⇣",
            diverged: "⇕",
            modified: "!",
            staged: "+",
            renamed: "»",
            deleted: "✘",
            untracked: "?",
            conflicted_count: true,
            ahead_behind: true,
            disabled: true,
        }
    }
}
