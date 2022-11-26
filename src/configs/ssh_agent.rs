use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct SshAgentConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub symbol_multi: &'a str,
    pub none_symbol: &'a str,
    pub style: &'a str,
    pub none_style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for SshAgentConfig<'a> {
    fn default() -> Self {
        Self {
            format: "[ssh $symbol]($style)",
            symbol: "🔑 ",
            symbol_multi: "🔑➕ ",
            none_symbol: "❌ ",
            style: "bold green",
            none_style: "bold red",
            disabled: true,
        }
    }
}
