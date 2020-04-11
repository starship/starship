use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitStatusConfig<'a> {
    pub format: &'a str,
    pub stashed_format: &'a str,
    pub ahead_format: &'a str,
    pub behind_format: &'a str,
    pub diverged_format: &'a str,
    pub conflicted_format: &'a str,
    pub deleted_format: &'a str,
    pub renamed_format: &'a str,
    pub modified_format: &'a str,
    pub staged_format: &'a str,
    pub untracked_format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitStatusConfig<'a> {
    fn new() -> Self {
        GitStatusConfig {
            format: r"\[[$all_status$ahead_behind](red bold)\] ",
            stashed_format: r"\$",
            ahead_format: "⇡",
            behind_format: "⇣",
            diverged_format: "⇕",
            conflicted_format: "=",
            deleted_format: "✘",
            renamed_format: "»",
            modified_format: "!",
            staged_format: "+",
            untracked_format: "?",
            disabled: false,
        }
    }
}
