use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ZluaConfig<'a> {
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub symbol: SegmentConfig<'a>,
    pub style: Style,
    pub disabled: bool,
    pub lua_exe_location: &'a str,
    pub zlua_script_location: &'a str,
}

impl<'a> RootModuleConfig<'a> for ZluaConfig<'a> {
    fn new() -> Self {
        ZluaConfig {
            prefix: "[z",
            suffix: "] ",
            symbol: SegmentConfig::new(""),
            style: Color::Cyan.bold(),
            disabled: false,
            lua_exe_location: "",
            zlua_script_location: "",
        }
    }
}
