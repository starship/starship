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
    pub format: &'a str,
    pub change_id_length: usize,
    pub disabled: bool,
    pub ignore_working_copy: bool,
}

impl Default for JujutsuChangeConfig<'_> {
    fn default() -> Self {
        Self {
            disabled: true,
            format: "[$symbol$change_id]($style) ",
            style: "purple",
            symbol: "",
            change_id_length: 7,
            ignore_working_copy: true,
        }
    }
}
