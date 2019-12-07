use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitCommitConfig<'a> {
    pub format: &'a str,
    pub commit_hash_length: usize,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitCommitConfig<'a> {
    fn new() -> Self {
        GitCommitConfig {
            format: "${styled?value=(&style=green bold}${hash?style=green bold}${styled?value=)&style=green bold} ",
            // be consistent with git by default, which has DEFAULT_ABBREV set to 7
            commit_hash_length: 7,
            disabled: true,
        }
    }
}
