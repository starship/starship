use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JjStatusConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub conflicted: &'a str,
    pub conflicted_style: &'a str,
    pub divergent: &'a str,
    pub divergent_style: &'a str,
    pub hidden: &'a str,
    pub hidden_style: &'a str,
    pub immutable: &'a str,
    pub immutable_style: &'a str,
    pub bookmark_conflicted: &'a str,
    pub bookmark_conflicted_style: &'a str,
    pub no_description: &'a str,
    pub disabled: bool,
}

impl Default for JjStatusConfig<'_> {
    fn default() -> Self {
        Self {
            conflicted: "⚠ ",
            conflicted_style: "red",
            disabled: true,
            divergent: "󰓁 ",
            divergent_style: "purple",
            format: "[$symbol$change_id( $bookmarks)]($style)[$conflicted]($conflicted_style)[$divergent]($divergent_style)[$hidden]($hidden_style)[$immutable]($immutable_style)[$bookmark_conflicted]($bookmark_conflicted_style) ",
            hidden: "󰘓 ",
            hidden_style: "none",
            immutable: " ",
            immutable_style: "red",
            bookmark_conflicted: "??",
            bookmark_conflicted_style: "red",
            no_description: "(no description)",
            style: "purple",
            symbol: "",
        }
    }
}
