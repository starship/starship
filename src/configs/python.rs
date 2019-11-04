use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct PythonConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub pyenv_prefix: SegmentConfig<'a>,
    pub pyenv_version_name: bool,
    pub display_virtualenv: bool,
    pub only_display_in_virtualenv: bool,
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
            display_virtualenv: true,
            only_display_in_virtualenv: false,
            style: Color::Yellow.bold(),
            disabled: false,
        }
    }
}
