use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct CharacterConfig<'a> {
    pub format: &'a str,
    pub success_symbol: &'a str,
    pub error_symbol: &'a str,
    #[serde(alias = "vicmd_symbol")]
    pub vimcmd_symbol: &'a str,
    pub vimcmd_visual_symbol: &'a str,
    pub vimcmd_replace_symbol: &'a str,
    pub vimcmd_replace_one_symbol: &'a str,
    pub vimcmd_insert_success_symbol: Option<&'a str>,
    pub vimcmd_insert_error_symbol: Option<&'a str>,
    pub disabled: bool,
}

impl<'a> Default for CharacterConfig<'a> {
    fn default() -> Self {
        CharacterConfig {
            format: "$symbol ",
            success_symbol: "[❯](bold green)",
            error_symbol: "[❯](bold red)",
            vimcmd_symbol: "[❮](bold green)",
            vimcmd_visual_symbol: "[❮](bold yellow)",
            vimcmd_replace_symbol: "[❮](bold purple)",
            vimcmd_replace_one_symbol: "[❮](bold purple)",
            vimcmd_insert_success_symbol: None,
            vimcmd_insert_error_symbol: None,
            disabled: false,
        }
    }
}
