use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitStatusConfig<'a> {
    pub format: &'a str,
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

impl<'a> RootModuleConfig<'a> for GitStatusConfig<'a> {
    fn new() -> Self {
        GitStatusConfig {
            format: r"([\[$all_status$ahead_behind\]](red bold) )",
            stashed: r"\$",
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
