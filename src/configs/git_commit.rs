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
}

impl<'a> RootModuleConfig<'a> for GitCommitConfig<'a> {
    fn new() -> Self {
        GitCommitConfig {
            hash: SegmentConfig::default(),
            prefix: "(",
            suffix: ") ",
            style: Color::Green.bold(),
            disabled: true,
        }
    }
}
