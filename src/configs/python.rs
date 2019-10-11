use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PythonConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub hide_version: bool,
    pub pyenv_version_name: bool,
    pub pyenv_prefix: SegmentConfig<'a>,
    pub virtual_env: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PythonConfig<'a> {
    fn new() -> Self {
        PythonConfig {
            symbol: SegmentConfig::new("🐍 "),
            version: SegmentConfig::default(),
            hide_version: false,
            pyenv_version_name: false,
            pyenv_prefix: SegmentConfig::new("pyenv "),
            virtual_env: SegmentConfig::default(),
            style: Color::Yellow.bold(),
            disabled: false,
        }
    }
}
