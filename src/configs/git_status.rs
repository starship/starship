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
    pub worktree_added: &'a str,
    pub worktree_deleted: &'a str,
    pub worktree_modified: &'a str,
    pub worktree_typechanged: &'a str,
    pub index_added: &'a str,
    pub index_deleted: &'a str,
    pub index_modified: &'a str,
    pub index_typechanged: &'a str,
    pub ignore_submodules: bool,
    pub disabled: bool,
    pub use_git_executable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_starship: Option<&'a str>,
}

impl Default for GitStatusConfig<'_> {
    fn default() -> Self {
        Self {
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
            worktree_added: "",
            worktree_deleted: "",
            worktree_modified: "",
            worktree_typechanged: "",
            index_added: "",
            index_deleted: "",
            index_modified: "",
            index_typechanged: "",
            ignore_submodules: false,
            disabled: false,
            use_git_executable: false,
            windows_starship: None,
        }
    }
}
