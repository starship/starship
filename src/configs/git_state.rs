use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitStateConfig<'a> {
    pub rebase: &'a str,
    pub merge: &'a str,
    pub revert: &'a str,
    pub cherry_pick: &'a str,
    pub bisect: &'a str,
    pub am: &'a str,
    pub am_or_rebase: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitStateConfig<'a> {
    fn new() -> Self {
        GitStateConfig {
            rebase: "REBASING",
            merge: "MERGING",
            revert: "REVERTING",
            cherry_pick: "CHERRY-PICKING",
            bisect: "BISECTING",
            am: "AM",
            am_or_rebase: "AM/REBASE",
            style: "bold yellow",
            format: "[\\($state( $progress_current/$progress_total)\\)]($style) ",
            disabled: false,
        }
    }
}
