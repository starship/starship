use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
pub struct CmdDurationConfig<'a> {
    pub min_time: i64,
    pub format: &'a str,
    pub style: &'a str,
    pub show_milliseconds: bool,
    pub disabled: bool,
    pub show_notifications: bool,
    pub min_time_to_notify: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_timeout: Option<u32>,
}

impl<'a> Default for CmdDurationConfig<'a> {
    fn default() -> Self {
        CmdDurationConfig {
            min_time: 2_000,
            format: "took [$duration]($style) ",
            show_milliseconds: false,
            style: "yellow bold",
            disabled: false,
            show_notifications: false,
            min_time_to_notify: 45_000,
            notification_timeout: None,
        }
    }
}
