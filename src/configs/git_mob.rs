use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GitMobConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
    pub separator: &'a str,
}

impl<'a> Default for GitMobConfig<'a> {
    fn default() -> Self {
        GitMobConfig {
            format: "[$symbol $initials \\($count\\)]($style)",
            style: "green bold",
            symbol: "👥",
            disabled: true,
            separator: ",",
        }
    }
}
