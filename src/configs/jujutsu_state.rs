use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JujutsuStateConfig<'a> {
    pub conflicted: &'a str,
    pub conflicted_style: &'a str,
    pub divergent: &'a str,
    pub divergent_style: &'a str,
    pub hidden: &'a str,
    pub hidden_style: &'a str,
    pub immutable: &'a str,
    pub immutable_style: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl Default for JujutsuStateConfig<'_> {
    fn default() -> Self {
        Self {
            disabled: true,
            format: "[$conflicted]($conflicted_style)[$divergent]($divergent_style)[$hidden]($hidden_style)[$immutable]($immutable_style)",
            style: "red",
            conflicted: "⚠ ",
            conflicted_style: "red",
            divergent: "󰓁 ",
            divergent_style: "purple",
            hidden: "󰘓 ",
            hidden_style: "none",
            immutable: " ",
            immutable_style: "red",
        }
    }
}
