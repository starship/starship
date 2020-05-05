use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PythonConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub pyenv_prefix: SegmentConfig<'a>,
    pub pyenv_version_name: bool,
    pub scan_for_pyfiles: bool,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for PythonConfig<'a> {
    fn new() -> Self {
        PythonConfig {
            symbol: SegmentConfig::new("🐍 "),
            version: SegmentConfig::default(),
            pyenv_prefix: SegmentConfig::new("pyenv "),
            pyenv_version_name: false,
            scan_for_pyfiles: true,
            style: Color::Yellow.bold(),
            disabled: false,
        }
    }
}
