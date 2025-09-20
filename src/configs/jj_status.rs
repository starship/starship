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
    pub ahead: &'a str,
    pub behind: &'a str,
    pub up_to_date: &'a str,
    pub diverged: &'a str,
    pub conflicted: &'a str,
    pub deleted: &'a str,
    pub renamed: &'a str,
    pub modified: &'a str,
    pub staged: &'a str,
    pub untracked: &'a str,
    pub disabled: bool,
}

impl Default for JjStatusConfig<'_> {
    fn default() -> Self {
        Self {
            format: "([\\[$all_status$ahead_behind\\]]($style) )",
            style: "red bold",
            ahead: "⇡",
            behind: "⇣",
            up_to_date: "",
            diverged: "⇕",
            conflicted: "=",
            deleted: "✘",
            renamed: "»",
            modified: "!",
            staged: "+",
            untracked: "?",
            disabled: false,
        }
    }
}