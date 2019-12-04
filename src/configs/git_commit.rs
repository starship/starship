use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitCommitConfig<'a> {
    pub hash: SegmentConfig<'a>,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
    pub commit_hash_length: usize,
}

impl<'a> RootModuleConfig<'a> for GitCommitConfig<'a> {
    fn new() -> Self {
        GitCommitConfig {
            hash: SegmentConfig::default(),
            prefix: "(",
            suffix: ") ",
            style: Color::Green.bold(),
            disabled: true,
            // be consistent with git by default, which has DEFAULT_ABBREV set to 7
            commit_hash_length: 7,
        }
    }
}
