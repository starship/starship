use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct NetnsConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl Default for NetnsConfig<'_> {
    fn default() -> Self {
        Self {
            format: "[$symbol \\[$name\\]]($style) ",
            symbol: "🛜",
            style: "blue bold dimmed",
            disabled: false,
        }
    }
}
