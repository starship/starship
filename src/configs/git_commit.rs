use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitCommitConfig<'a> {
    pub commit_hash_length: usize,
    pub format: &'a str,
    pub style: &'a str,
    pub only_detached: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitCommitConfig<'a> {
    fn new() -> Self {
        GitCommitConfig {
            // be consistent with git by default, which has DEFAULT_ABBREV set to 7
            commit_hash_length: 7,
            format: "[\\($hash\\)]($style) ",
            style: "green bold",
            only_detached: true,
            disabled: false,
        }
    }
}
