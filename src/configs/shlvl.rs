use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ShLvlConfig<'a> {
    pub symbol: Option<SegmentConfig<'a>>,
    pub threshold: i64,
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for ShLvlConfig<'a> {
    fn new() -> Self {
        ShLvlConfig {
            threshold: 2,
            format: "[$symbol$shlvl]($style) ",
            symbol: "↕️  ", // extra space for emoji
            style: "bold yellow",
            disabled: true,
        }
    }
}
