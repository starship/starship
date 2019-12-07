use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitStatusConfig<'a> {
    pub format: &'a str,
    pub show_sync_count: bool,
    pub stashed_format: &'a str,
    pub diverged_format: &'a str,
    pub ahead_format: &'a str,
    pub behind_format: &'a str,
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
            format: "\
                     ${styled?value=[&style=red bold}\
                     ${stashed?style=red bold}\
                     ${diverged?style=red bold}\
                     ${ahead?style=red bold}\
                     ${behind?style=red bold}\
                     ${deleted?style=red bold}\
                     ${renamed?style=red bold}\
                     ${modified?style=red bold}\
                     ${staged?style=red bold}\
                     ${untracked?style=red bold}\
                     ${styled?value=]&style=red bold} \
                     ",
            show_sync_count: false,
            stashed_format: "$",
            diverged_format: "⇕",
            ahead_format: "⇡",
            behind_format: "⇣",
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
