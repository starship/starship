use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GitTagsConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub separator: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for GitTagsConfig<'a> {
    fn default() -> Self {
        Self {
            format: "[\\($symbol$tags\\)]($style) ",
            symbol: "🏷 ",
            separator: " ",
            style: "yellow bold",
            disabled: false,
        }
    }
}
