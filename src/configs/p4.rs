use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct P4Config<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> Default for P4Config<'a> {
    fn default() -> Self {
        P4Config {
            format: "[$symbol $user@$client#$changelist]($style) ",
            symbol: "p4",
            style: "bold blue",
            disabled: false,
        }
    }
}
