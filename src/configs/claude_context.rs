use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct ClaudeContextConfig<'a> {
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

impl Default for ClaudeContextConfig<'_> {
    fn default() -> Self {
        Self {
            format: "[$gauge $percentage]($style) ",
            symbol: "",
            gauge_width: 5,
            gauge_full_symbol: "▰",
            gauge_partial_symbol: "",
            gauge_empty_symbol: "▱",
            display: vec![
                ClaudeDisplayConfig {
                    threshold: 0.,
                    hidden: true,
                    ..Default::default()
                },
                ClaudeDisplayConfig {
                    threshold: 30.,
                    style: "bold green",
                    ..Default::default()
                },
                ClaudeDisplayConfig {
                    threshold: 60.,
                    style: "bold yellow",
                    ..Default::default()
                },
                ClaudeDisplayConfig {
                    threshold: 80.,
                    style: "bold red",
                    ..Default::default()
                },
            ],
            disabled: false,
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
pub struct ClaudeDisplayConfig<'a> {
    pub threshold: f32,
    pub style: &'a str,
    pub hidden: bool,
}

impl Default for ClaudeDisplayConfig<'_> {
    fn default() -> Self {
        Self {
            threshold: 0.,
            style: "bold green",
            hidden: false,
        }
    }
}
