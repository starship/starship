use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct SingularityConfig<'a> {
    pub symbol: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for SingularityConfig<'a> {
    fn default() -> Self {
        SingularityConfig {
            format: "[$symbol\\[$env\\]]($style) ",
            symbol: "",
            style: "blue bold dimmed",
            disabled: false,
        }
    }
}
