use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct TimeConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub use_12hr: bool,
    pub time_format: Option<&'a str>,
    pub utc_time_offset: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for TimeConfig<'a> {
    fn new() -> Self {
        TimeConfig {
            format: "at [$time]($style) ",
            style: "bold yellow",
            use_12hr: false,
            time_format: None,
            utc_time_offset: "local",
            disabled: true,
        }
    }
}
