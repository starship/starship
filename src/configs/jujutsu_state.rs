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
    pub empty: &'a str,
    pub empty_style: &'a str,
    pub hidden: &'a str,
    pub hidden_style: &'a str,
    pub immutable: &'a str,
    pub immutable_style: &'a str,
    pub missing_description: &'a str,
    pub missing_description_style: &'a str,
    pub format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
}

impl Default for JujutsuStateConfig<'_> {
    fn default() -> Self {
        Self {
            disabled: false,
            format: "(\\[[$conflicted]($conflicted_style)[$divergent]($divergent_style)[$empty](empty_style)[$hidden]($hidden_style)[$immutable]($immutable_style)[$missing_description]($missing_description_style)\\])",
            style: "red",
            conflicted: "⚠",
            conflicted_style: "red",
            divergent: "󰓁",
            divergent_style: "purple",
            empty: "∅",
            empty_style: "orange",
            hidden: "󰘓",
            hidden_style: "none",
            immutable: "",
            immutable_style: "red",
            missing_description: "",
            missing_description_style: "red",
        }
    }
}
