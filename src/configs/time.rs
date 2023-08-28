use crate::config::Either;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct TimeConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub use_12hr: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_format: Option<&'a str>,
    pub disabled: bool,
    #[cfg_attr(feature = "config-schema", schemars(with = "String"))]
    pub utc_time_offset: Either<chrono_tz::Tz, &'a str>,
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
            utc_time_offset: Either::Second("local"),
            time_range: "-",
        }
    }
}
