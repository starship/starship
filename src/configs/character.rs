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
    pub cancel_symbol: &'a str,
    pub error_symbol: &'a str,
    #[serde(alias = "vicmd_symbol")]
    pub vimcmd_success_symbol: &'a str,
    pub vimcmd_visual_symbol: &'a str,
    pub vimcmd_replace_symbol: &'a str,
    pub vimcmd_replace_one_symbol: &'a str,
    pub vimcmd_cancel_symbol: &'a str,
    pub vimcmd_error_symbol: &'a str,
    pub disabled: bool,
}

impl<'a> Default for CharacterConfig<'a> {
    fn default() -> Self {
        CharacterConfig {
            format: "$symbol ",
            success_symbol: "[❯](bold green)",
            cancel_symbol: "[❯](bold yellow)",
            error_symbol: "[❯](bold red)",
            vimcmd_visual_symbol: "[❮](bold yellow)",
            vimcmd_replace_symbol: "[❮](bold purple)",
            vimcmd_replace_one_symbol: "[❮](bold purple)",
            vimcmd_success_symbol: "[❮](bold green)",
            vimcmd_cancel_symbol: "[❮](bold yellow)",
            vimcmd_error_symbol: "[❮](bold red)",
            disabled: false,
        }
    }
}
