use crate::config::Either;
use serde::{Deserialize, Serialize};

// Wrapper struct to enable serde serialization/deserialization for jiff::tz::TimeZone
#[derive(Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct TimezoneWrapper(
    #[serde(with = "jiff::fmt::serde::tz::required")] pub jiff::tz::TimeZone,
);

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
    pub utc_time_offset: Either<TimezoneWrapper, &'a str>,
    pub time_range: &'a str,
}

impl Default for TimeConfig<'_> {
    fn default() -> Self {
        Self {
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
