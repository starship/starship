use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitStateConfig<'a> {
    pub format: &'a str,
    pub progress_format: &'a str,
    pub rebase: &'a str,
    pub merge: &'a str,
    pub revert: &'a str,
    pub cherry_pick: &'a str,
    pub bisect: &'a str,
    pub am: &'a str,
    pub am_or_rebase: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitStateConfig<'a> {
    fn new() -> Self {
        GitStateConfig {
            format: "(${label?style=yellow bold}${progress?style=yellow bold}) ",
            progress_format: " ${current}/${total}",
            rebase: "REBASING",
            merge: "MERGING",
            revert: "REVERTING",
            cherry_pick: "CHERRY-PICKING",
            bisect: "BISECTING",
            am: "AM",
            am_or_rebase: "AM/REBASE",
            disabled: false,
        }
    }
}
