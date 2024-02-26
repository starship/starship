use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct FossilMetricsConfig<'a> {
    pub format: &'a str,
    pub added_style: &'a str,
    pub deleted_style: &'a str,
    pub only_nonzero_diffs: bool,
    pub disabled: bool,
}

impl<'a> Default for FossilMetricsConfig<'a> {
    fn default() -> Self {
        FossilMetricsConfig {
            format: "([+$added]($added_style) )([-$deleted]($deleted_style) )",
            added_style: "bold green",
            deleted_style: "bold red",
            only_nonzero_diffs: true,
            disabled: true,
        }
    }
}
