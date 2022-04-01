use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct CharacterConfig<'a> {
    pub format: &'a str,
    pub success_symbol: &'a str,
    pub error_symbol: &'a str,
    pub vicmd_symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for CharacterConfig<'a> {
    fn default() -> Self {
        CharacterConfig {
            format: "$symbol ",
            success_symbol: "[❯](bold green)",
            error_symbol: "[❯](bold red)",
            vicmd_symbol: "[❮](bold green)",
            disabled: false,
        }
    }
}
