use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JujutsuChangeConfig<'a> {
    pub symbol: &'a str,
    pub prefix_style: &'a str,
    pub suffix_style: &'a str,
    pub format: &'a str,
    pub change_id_length: usize,
    pub disabled: bool,
}

impl Default for JujutsuChangeConfig<'_> {
    fn default() -> Self {
        Self {
            disabled: false,
            format: "[$change_prefix]($prefix_style)$change_suffix($suffix_style) ",
            prefix_style: "purple",
            suffix_style: "",
            symbol: "",
            change_id_length: 7,
        }
    }
}
