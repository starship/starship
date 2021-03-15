use crate::config::ModuleConfig;

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct TimeConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub use_12hr: bool,
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
