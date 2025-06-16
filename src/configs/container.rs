use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct ContainerConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl Default for ContainerConfig<'_> {
    fn default() -> Self {
        ContainerConfig {
            format: "[$symbol \\[$name\\]]($style) ",
            symbol: "⬢",
            style: "red bold dimmed",
            disabled: false,
        }
    }
}
