use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct UptermConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl Default for UptermConfig<'_> {
    fn default() -> Self {
        Self {
            format: "via [$symbol]($style) ",
            symbol: "🆙 ",
            style: "cyan bold",
            disabled: false,
        }
    }
}
