use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

use toml::Value;

#[derive(Clone, ModuleConfig)]
pub struct PythonConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub pyenv_prefix: SegmentConfig<'a>,
    pub pyenv_version_name: bool,
    pub show_venv: ShowVenv,
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
            show_venv: ShowVenv::NotInNix,
            style: Color::Yellow.bold(),
            disabled: false,
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum ShowVenv {
    Always,
    Never,
    NotInNix,
}

impl<'a> ModuleConfig<'a> for ShowVenv {
    fn from_config(config: &Value) -> Option<Self> {
        match config.as_str() {
            Some("always") => Some(ShowVenv::Always),
            Some("never") => Some(ShowVenv::Never),
            Some("not_in_nix") => Some(ShowVenv::NotInNix),
            _ => None,
        }
    }
}
