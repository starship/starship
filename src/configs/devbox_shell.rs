use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct DevboxShellConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for DevboxShellConfig<'a> {
    fn default() -> Self {
        DevboxShellConfig {
            format: "via [$symbol]($style) ",
            symbol: "</> ",
            style: "Purple bold",
            disabled: false,
        }
    }
}
