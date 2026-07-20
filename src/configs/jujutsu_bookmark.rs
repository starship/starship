use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JujutsuBookmarkConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub bookmark_format: &'a str,
    pub ahead: &'a str,
    pub behind: &'a str,
    pub diverged: &'a str,
    pub up_to_date: &'a str,
    pub conflicted: &'a str,
    pub joiner: &'a str,
    pub find_closest: bool,
    pub disabled: bool,
}

impl Default for JujutsuBookmarkConfig<'_> {
    fn default() -> Self {
        Self {
            disabled: true,
            format: "[$symbol$bookmarks]($style) ",
            bookmark_format: "$bookmark_name$ahead_behind$conflicted",
            ahead: " ⇡$count",
            behind: " ⇣$count",
            diverged: " ⇡$ahead_count⇣$behind_count",
            up_to_date: " ✓",
            conflicted: "??",
            joiner: " ",
            style: "purple",
            symbol: "󰑟 ",
            find_closest: false,
        }
    }
}
