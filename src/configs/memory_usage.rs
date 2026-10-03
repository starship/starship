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
    pub show_decimals: bool,
    pub format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
}

impl Default for MemoryConfig<'_> {
    fn default() -> Self {
        Self {
            threshold: 75,
            show_decimals: false,
            format: "via $symbol[$ram( | $swap)]($style) ",
            style: "white bold dimmed",
            symbol: "🐏 ",
            disabled: true,
        }
    }
}
