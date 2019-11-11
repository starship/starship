use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct TerraformConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub workspace: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for TerraformConfig<'a> {
    fn new() -> Self {
        TerraformConfig {
            symbol: SegmentConfig::new("🛠️ "),
            workspace: SegmentConfig::default(),
            style: Color::Fixed(105).bold(),
            disabled: false,
        }
    }
}
