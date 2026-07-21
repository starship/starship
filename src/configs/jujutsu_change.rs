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
    pub style: &'a str,
    pub prefix_style: &'a str,
    pub description_style: &'a str,
    pub description_limit: usize,
    pub no_description_symbol: &'a str,
    pub format: &'a str,
    pub change_id_length: usize,
    pub disabled: bool,
}

impl Default for JujutsuChangeConfig<'_> {
    fn default() -> Self {
        Self {
            disabled: false,
            format: "on [$symbol [$change_prefix]($prefix_style)$change_suffix]($style)([ ('$description')$no_description]($description_style)) ",
            prefix_style: "purple",
            style: "green",
            description_style: "",
            description_limit: 15,
            no_description_symbol: "",
            symbol: "󱗆",
            change_id_length: 7,
        }
    }
}
