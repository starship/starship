use crate::config::ModuleConfig;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitCommitConfig<'a> {
    pub commit_hash_length: usize,
    pub format: &'a str,
    pub style: &'a str,
    pub only_detached: bool,
    pub disabled: bool,
    pub tag_symbol: &'a str,
    pub tag_disabled: bool,
}

impl<'a> Default for GitCommitConfig<'a> {
    fn default() -> Self {
        GitCommitConfig {
            // be consistent with git by default, which has DEFAULT_ABBREV set to 7
            commit_hash_length: 7,
            format: "[\\($hash$tag\\)]($style) ",
            style: "green bold",
            only_detached: true,
            disabled: false,
            tag_symbol: "🏷  ",
            tag_disabled: true,
        }
    }
}
