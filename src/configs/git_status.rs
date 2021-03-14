use crate::config::ModuleConfig;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitStatusConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub stashed: &'a str,
    pub ahead: &'a str,
    pub behind: &'a str,
    pub diverged: &'a str,
    pub conflicted: &'a str,
    pub deleted: &'a str,
    pub renamed: &'a str,
    pub modified: &'a str,
    pub staged: &'a str,
    pub untracked: &'a str,
    pub disabled: bool,
}

impl<'a> Default for GitStatusConfig<'a> {
    fn default() -> Self {
        GitStatusConfig {
            format: "([\\[$all_status$ahead_behind\\]]($style) )",
            style: "red bold",
            stashed: "\\$",
            ahead: "⇡",
            behind: "⇣",
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
