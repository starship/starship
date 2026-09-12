use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JJMetricsConfig<'a> {
    /// Value of the `$added_style` variable in the format string
    pub added_style: &'a str,
    /// Value of the `$deleted_style` variable in the format string
    pub deleted_style: &'a str,
    /// Render only for changed items
    pub only_nonzero_diffs: bool,
    /// Format string for the module
    pub format: &'a str,
    /// Disable the `jj_metrics`
    pub disabled: bool,
}

impl Default for JJMetricsConfig<'_> {
    fn default() -> Self {
        Self {
            added_style: "bold green",
            deleted_style: "bold red",
            only_nonzero_diffs: true,
            format: "([+$added]($added_style) )([-$deleted]($deleted_style) )",
            disabled: false,
        }
    }
}
