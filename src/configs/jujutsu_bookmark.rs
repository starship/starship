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
    pub joiner: &'a str,
    pub max_length: usize,
    pub max_depth: usize,
    pub max_shown: usize,
    pub disabled: bool,
}

impl Default for JujutsuBookmarkConfig<'_> {
    fn default() -> Self {
        Self {
            disabled: true,
            format: "[$symbol$bookmarks]($style) ",
            bookmark_format: "$bookmark_name(~$distance)$ahead_behind",
            ahead: " ⇡$count",
            behind: " ⇣$count",
            diverged: " ⇡$ahead_count⇣$behind_count",
            up_to_date: " ✓",
            joiner: " ",
            style: "cyan",
            symbol: "󰑟 ",
            max_depth: 10,
            max_shown: 3,
            max_length: 15,
        }
    }
}
