use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct UnityConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for UnityConfig<'a> {
    fn default() -> Self {
        UnityConfig {
            format: "via [$symbol$version ]($style)",
            version_format: "v${raw}",
            symbol: "🎮 ",
            style: "bold blue",
            disabled: false,
        }
    }
}
