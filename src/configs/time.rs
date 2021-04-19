use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct TimeConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub use_12hr: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_format: Option<&'a str>,
    pub disabled: bool,
    pub utc_time_offset: &'a str,
    pub time_range: &'a str,
}

impl<'a> Default for TimeConfig<'a> {
    fn default() -> Self {
        TimeConfig {
            format: "at [$time]($style) ",
            style: "bold yellow",
            use_12hr: false,
            time_format: None,
            disabled: true,
            utc_time_offset: "local",
            time_range: "-",
        }
    }
}
