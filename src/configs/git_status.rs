use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GitStatusConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub stashed: &'a str,
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
    pub typechanged: &'a str,
    pub ignore_submodules: bool,
    pub disabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_starship: Option<&'a str>,
    pub git_version: &'a str,
}

impl<'a> Default for GitStatusConfig<'a> {
    fn default() -> Self {
        GitStatusConfig {
            format: "([\\[$all_status$ahead_behind\\]]($style) )",
            style: "red bold",
            stashed: "\\$",
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
            typechanged: "",
            ignore_submodules: false,
            disabled: false,
            windows_starship: None,
            git_version: "2.11",
        }
    }
}
