use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct GitStatusConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub stashed: &'a str,
    pub ahead: &'a str,
    pub behind: &'a str,
    pub up_to_date: &'a str,
    pub diverged: &'a str,
    pub conflicted: &'a str,
    pub work_dir: &'a str,
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
            up_to_date: "",
            diverged: "⇕",
            conflicted: "=",
            work_dir: "!",
            staged: "+",
            untracked: "?",
            disabled: false,
        }
    }
}
