use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GuixShellConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl<'a> Default for GuixShellConfig<'a> {
    fn default() -> Self {
        GuixShellConfig {
            format: "via [$symbol]($style) ",
            symbol: "🐃 ",
            style: "yellow bold",
            disabled: false,
        }
    }
}
