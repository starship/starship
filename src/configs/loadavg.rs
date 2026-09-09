use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct LoadavgConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    #[serde(borrow)]
    pub display: Vec<LoadavgDisplayConfig<'a>>,
    pub disabled: bool,
}

impl<'a> Default for LoadavgConfig<'a> {
    fn default() -> Self {
        LoadavgConfig {
            format: "[$symbol $one $five $fifteen]($style) ",
            symbol: "⌛",
            display: vec![LoadavgDisplayConfig::default()],
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
#[serde(default)]
pub struct LoadavgDisplayConfig<'a> {
    pub threshold_one: f32,
    pub threshold_five: f32,
    pub threshold_fifteen: f32,
    pub style: &'a str,
    pub symbol: Option<&'a str>,
}

impl<'a> Default for LoadavgDisplayConfig<'a> {
    fn default() -> Self {
        LoadavgDisplayConfig {
            threshold_one: f32::NAN,
            threshold_five: f32::NAN,
            threshold_fifteen: f32::NAN,
            style: "white bold",
            symbol: None,
        }
    }
}
