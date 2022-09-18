use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct SudoConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub allow_windows: bool,
    pub disabled: bool,
}

impl<'a> Default for SudoConfig<'a> {
    fn default() -> Self {
        SudoConfig {
            format: "[as $symbol]($style)",
            symbol: "🧙 ",
            style: "bold blue",
            allow_windows: false,
            disabled: true,
        }
    }
}
