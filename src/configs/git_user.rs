use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GitUserConfig<'a> {
    pub format: &'a str,
    pub style: &'a str,
    pub truncation_length: i64,
    pub truncation_symbol: &'a str,
    pub show_always: bool,
    pub disabled: bool,
}

impl Default for GitUserConfig<'_> {
    fn default() -> Self {
        Self {
            format: "as [$name( \\($email\\))]($style) ",
            style: "bold green",
            truncation_length: i64::MAX,
            truncation_symbol: "…",
            show_always: false,
            disabled: false,
        }
    }
}
