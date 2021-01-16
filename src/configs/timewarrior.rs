use crate::config::{ModuleConfig, RootModuleConfig};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct TimewarriorConfig<'a> {
    pub symbol: &'a str,
    pub symbol_style: &'a str,
    pub tags_style: &'a str,
    pub show_tags: bool,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for TimewarriorConfig<'a> {
    fn new() -> Self {
        TimewarriorConfig {
            symbol: "",
            symbol_style: "white",
            tags_style: "white",
            show_tags: true,
            disabled: true,
        }
    }
}
