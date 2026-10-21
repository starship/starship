use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields, transform = add_legacy_vimcmd_symbols)
)]
#[serde(default)]
pub struct CharacterConfig<'a> {
    pub format: &'a str,
    pub success_symbol: &'a str,
    pub cancel_symbol: &'a str,
    pub error_symbol: &'a str,
    #[serde(alias = "vimcmd_symbol", alias = "vicmd_symbol")]
    pub vimcmd_success_symbol: &'a str,
    pub vimcmd_cancel_symbol: &'a str,
    pub vimcmd_error_symbol: &'a str,
    pub vimcmd_visual_symbol: &'a str,
    pub vimcmd_replace_symbol: &'a str,
    pub vimcmd_replace_one_symbol: &'a str,
    pub disabled: bool,
}

impl Default for CharacterConfig<'_> {
    fn default() -> Self {
        Self {
            format: "$symbol ",
            success_symbol: "[❯](bold green)",
            cancel_symbol: "[❯](bold yellow)",
            error_symbol: "[❯](bold red)",
            vimcmd_success_symbol: "[❮](bold green)",
            vimcmd_cancel_symbol: "[❮](bold yellow)",
            vimcmd_error_symbol: "[❮](bold red)",
            vimcmd_visual_symbol: "[❮](bold yellow)",
            vimcmd_replace_symbol: "[❮](bold purple)",
            vimcmd_replace_one_symbol: "[❮](bold purple)",
            disabled: false,
        }
    }
}

#[cfg(feature = "config-schema")]
fn add_legacy_vimcmd_symbols(schema: &mut schemars::Schema) {
    for legacy in ["vimcmd_symbol", "vicmd_symbol"] {
        if let Some(properties) = schema
            .get_mut("properties")
            .and_then(serde_json::Value::as_object_mut)
        {
            properties.insert(
                legacy.to_owned(),
                serde_json::json!({
                    "type": "string",
                    "deprecated": true,
                    "description": "Deprecated alias of `vimcmd_success_symbol`."
                }),
            );
        }
    }
}
