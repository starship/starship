use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct ClaudeModelConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub model_aliases: IndexMap<String, &'a str>,
    pub disabled: bool,
}

impl Default for ClaudeModelConfig<'_> {
    fn default() -> Self {
        Self {
            format: "[$symbol$model]($style) ",
            symbol: "🤖 ",
            style: "bold blue",
            model_aliases: IndexMap::new(),
            disabled: false,
        }
    }
}
