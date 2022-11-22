use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct VcshConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> Default for VcshConfig<'a> {
    fn default() -> Self {
        VcshConfig {
            symbol: "",
            style: "bold yellow",
            format: "vcsh [$symbol$repo]($style) ",
            disabled: false,
        }
    }
}
