use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JujutsuClosestBookmarksConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub bookmark_conflicted: &'a str,
    pub disabled: bool,
    pub ignore_working_copy: bool,
}

impl Default for JujutsuClosestBookmarksConfig<'_> {
    fn default() -> Self {
        Self {
            disabled: true,
            format: "[$symbol$bookmarks]($style) ",
            style: "purple",
            symbol: "󰑟 ",
            bookmark_conflicted: "??",
            ignore_working_copy: true,
        }
    }
}
