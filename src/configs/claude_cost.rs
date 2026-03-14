use serde::{Deserialize, Serialize};

use crate::configs::claude_context::ClaudeDisplayConfig;

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct ClaudeCostConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    #[serde(borrow)]
    pub display: Vec<ClaudeDisplayConfig<'a>>,
    pub disabled: bool,
}

impl Default for ClaudeCostConfig<'_> {
    fn default() -> Self {
        Self {
            format: "[$symbol(\\$$cost)]($style) ",
            symbol: "💰 ",
            display: vec![
                ClaudeDisplayConfig {
                    threshold: 0.00,
                    hidden: true,
                    ..Default::default()
                },
                ClaudeDisplayConfig {
                    threshold: 1.0,
                    style: "bold yellow",
                    ..Default::default()
                },
                ClaudeDisplayConfig {
                    threshold: 5.0,
                    style: "bold red",
                    ..Default::default()
                },
            ],
            disabled: false,
        }
    }
}
