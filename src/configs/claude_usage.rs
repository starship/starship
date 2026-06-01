use serde::{Deserialize, Serialize};

use crate::configs::claude_context::ClaudeDisplayConfig;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct ClaudeUsageConfig<'a> {
    pub format: &'a str,
    #[serde(borrow)]
    pub display: Vec<ClaudeDisplayConfig<'a>>,
    pub disabled: bool,
}

impl Default for ClaudeUsageConfig<'_> {
    fn default() -> Self {
        Self {
            format: "[($five_hour_pct% \\(resets in $five_hour_reset\\)  )($seven_day_pct% \\(resets in $seven_day_reset\\)]($style) )",
            display: vec![
                ClaudeDisplayConfig {
                    threshold: 0.0,
                    hidden: true,
                    ..Default::default()
                },
                ClaudeDisplayConfig {
                    threshold: 70.0,
                    style: "bold yellow",
                    ..Default::default()
                },
                ClaudeDisplayConfig {
                    threshold: 90.0,
                    style: "bold red",
                    ..Default::default()
                },
            ],
            disabled: false,
        }
    }
}
