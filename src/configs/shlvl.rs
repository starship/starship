use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct ShLvlConfig<'a> {
    pub threshold: i64,
    pub format: &'a str,
    pub symbol: &'a str,
    pub repeat: bool,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for ShLvlConfig<'a> {
    fn default() -> Self {
        ShLvlConfig {
            threshold: 2,
            format: "[$symbol$shlvl]($style) ",
            symbol: "↕️  ", // extra space for emoji
            repeat: false,
            style: "bold yellow",
            disabled: true,
        }
    }
}
