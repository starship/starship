use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JujutsuChangeConfig<'a> {
    pub change_id_length: usize,

    pub disabled: bool,
    pub format: &'a str,
    pub style: &'a str,
}

impl Default for JujutsuChangeConfig<'_> {
    fn default() -> Self {
        Self {
            // Copy the git default
            change_id_length: 7,

            disabled: true,
            format: "[\\($change_id\\)]($style) ",
            style: "bold purple",
        }
    }
}
