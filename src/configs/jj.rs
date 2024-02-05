use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JJConfig<'a> {
    pub symbol: &'a str,
    pub format: &'a str,
    pub disabled: bool,
    pub change_id_prefix_style: &'a str,
    pub change_id_suffix_style: &'a str,
    pub commit_id_prefix_style: &'a str,
    pub commit_id_suffix_style: &'a str,
    pub truncation_length: u8,
    pub no_description_symbol: &'a str,
    pub divergent_symbol: &'a str,
    pub branch_style: &'a str,
}

impl<'a> Default for JJConfig<'a> {
    fn default() -> Self {
        JJConfig {
            symbol: "🍐 ",
            format: "\\[$symbol[$change_id_prefix]($change_id_prefix_style)[$change_id_suffix]($change_id_suffix_style) [$commit_id_prefix]($commit_id_prefix_style)[$commit_id_suffix]($commit_id_suffix_style) on [$branch]($branch_style)$no_description_symbol$divergent_symbol\\]",
            disabled: false,
            change_id_prefix_style: "purple", // magenta
            change_id_suffix_style: "bright-black",
            commit_id_prefix_style: "blue",
            commit_id_suffix_style: "bright-black",
            truncation_length: 8,
            no_description_symbol: " 📝",
            divergent_symbol: " 💥",
            branch_style: "purple",
        }
    }
}
