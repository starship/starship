use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub enum TimeFormat {
    None,
    Austin,
    Roundrock,
    Dallas,
    Galveston,
    Galvestonms,
    Houston,
    Amarillo,
    Round,
    Lucky7,
}

impl From<&str> for TimeFormat {
    fn from(value: &str) -> Self {
        match value {
            "None" | "none" => TimeFormat::None,
            "Austin" | "austin" => TimeFormat::Austin,
            "Roundrock" | "roundrock" => TimeFormat::Roundrock,
            "Dallas" | "dallas" => TimeFormat::Dallas,
            "Galveston" | "galveston" => TimeFormat::Galveston,
            "Galvestonms" | "galvestonms" => TimeFormat::Galvestonms,
            "Houston" | "houston" => TimeFormat::Houston,
            "Amarillo" | "amarillo" => TimeFormat::Amarillo,
            "Round" | "round" => TimeFormat::Round,
            "Lucky7" | "lucky7" => TimeFormat::Lucky7,
            s => {
                log::warn!("time_format in [cmd_duration] ({}) is unknown", s);
                TimeFormat::None
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct CmdDurationConfig<'a> {
    pub min_time: i64,
    pub format: &'a str,
    pub style: &'a str,
    pub show_milliseconds: bool,
    pub disabled: bool,
    pub show_notifications: bool,
    pub min_time_to_notify: i64,
    pub time_format: &'a str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_timeout: Option<u32>,
}

impl Default for CmdDurationConfig<'_> {
    fn default() -> Self {
        CmdDurationConfig {
            min_time: 2_000,
            format: "took [$duration]($style) ",
            show_milliseconds: false,
            style: "yellow bold",
            disabled: false,
            show_notifications: false,
            min_time_to_notify: 45_000,
            time_format: "None",
            notification_timeout: None,
        }
    }
}
