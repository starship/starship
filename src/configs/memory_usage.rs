use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct MemoryConfig<'a> {
    pub threshold: i64,
    pub format: &'a str,
    pub style: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_style: Option<&'a str>,
    pub symbol: &'a str,
    pub thresholds: Option<Vec<ThresholdConfig<'a>>>,
    pub show_always: bool,
    pub disabled: bool,
}

impl Default for MemoryConfig<'_> {
    fn default() -> Self {
        Self {
            threshold: 75,
            format: "via $symbol[$ram( | $swap)]($style) ",
            style: "white bold dimmed",
            threshold_style: None,
            symbol: "🐏 ",
            thresholds: None,
            show_always: false,
            disabled: true,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
pub struct ThresholdConfig<'a> {
    pub value: i64,
    pub style: &'a str,
}
