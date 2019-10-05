use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct EnvVarConfig<'a> {
    pub symbol: Option<SegmentConfig<'a>>,
    pub variable: Option<&'a str>,
    pub default: Option<&'a str>,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for EnvVarConfig<'a> {
    fn new() -> Self {
        EnvVarConfig {
            symbol: None,
            variable: None,
            default: None,
            prefix: "",
            suffix: "",
            style: Color::Black.bold().dimmed(),
            disabled: false,
        }
    }
}
