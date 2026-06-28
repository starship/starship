use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct YadmConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub repo_path: Option<&'a str>,
    pub disabled: bool,
}

impl<'a> Default for YadmConfig<'a> {
    fn default() -> Self {
        YadmConfig {
            format: "[$symbol]($style)",
            symbol: " ",
            style: "red",
            repo_path: None,
            disabled: false,
        }
    }
}
