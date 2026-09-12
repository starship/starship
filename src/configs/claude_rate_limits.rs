use serde::{Deserialize, Serialize};

use crate::configs::claude_context::ClaudeDisplayConfig;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct ClaudeRateLimitsConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub gauge_width: u8,
    pub gauge_full_symbol: &'a str,
    pub gauge_partial_symbol: &'a str,
    pub gauge_empty_symbol: &'a str,
    #[serde(borrow)]
    pub display: Vec<ClaudeDisplayConfig<'a>>,
    pub disabled: bool,
}

impl Default for ClaudeRateLimitsConfig<'_> {
    fn default() -> Self {
        Self {
            format: "[$symbol( 5h $five_hour_percentage)( 7d $seven_day_percentage)]($style) ",
            symbol: "⏳",
            gauge_width: 5,
            gauge_full_symbol: "█",
            gauge_partial_symbol: "▒",
            gauge_empty_symbol: "░",
            display: vec![
                ClaudeDisplayConfig {
                    threshold: 0.,
                    hidden: true,
                    ..Default::default()
                },
                ClaudeDisplayConfig {
                    threshold: 50.,
                    style: "bold green",
                    ..Default::default()
                },
                ClaudeDisplayConfig {
                    threshold: 70.,
                    style: "bold yellow",
                    ..Default::default()
                },
                ClaudeDisplayConfig {
                    threshold: 90.,
                    style: "bold red",
                    ..Default::default()
                },
            ],
            disabled: false,
        }
    }
}
