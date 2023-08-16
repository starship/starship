use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct EnvVarConfig<'a> {
    pub symbol: &'a str,
    pub style: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<&'a str>,
    pub format: &'a str,
    pub disabled: bool,
    pub description: &'a str,
}

impl<'a> Default for EnvVarConfig<'a> {
    fn default() -> Self {
        EnvVarConfig {
            symbol: "",
            style: "black bold dimmed",
            variable: None,
            default: None,
            format: "with [$env_value]($style) ",
            disabled: false,
            description: "<env_var module>",
        }
    }
}
