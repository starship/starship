use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct GitStatusConfig<'a> {
    pub staged: SegmentConfig<'a>,
    pub staged_count_disabled: bool,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for GitStatusConfig<'a> {
    fn new() -> Self {
        GitStatusConfig {
            staged: SegmentConfig::new("+"),
            staged_count_disabled: true,
            style: Color::Red.bold(),
            disabled: false,
        }
    }
}
