use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct SshAgentConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub symbol_multi: &'a str,
    pub symbol_none: &'a str,
    pub style: &'a str,
    pub style_none: &'a str,
    pub disabled: bool,
}

impl<'a> Default for SshAgentConfig<'a> {
    fn default() -> Self {
        Self {
            format: "[ssh $symbol]($style)",
            symbol: "🔑 ",
            symbol_multi: "🔑➕ ",
            symbol_none: "❌ ",
            style: "bold green",
            style_none: "bold red",
            disabled: true,
        }
    }
}
