use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JJChangeConfig<'a> {
    /// Format string for the module
    pub format: &'a str,
    /// Value of the `$prefix_style` variable in the format string
    pub prefix_style: &'a str,
    /// Value of the `$suffix_style` variable in the format string
    pub suffix_style: &'a str,
    /// The max length of the displayed change hash, when combining prefix and suffix
    ///
    /// The actual length of the displayed value will be `max(prefix.len(), change_hash_length)`
    pub change_hash_length: u8,
    /// The max length of the displayed commit hash, when combining prefix and suffix
    ///
    /// The actual length of the displayed value will be `max(prefix.len(), commit_hash_length)`
    pub commit_hash_length: u8,
    /// Disable the module
    pub disabled: bool,
}

impl Default for JJChangeConfig<'_> {
    fn default() -> Self {
        Self {
            format: "$change ",
            prefix_style: "green bold",
            suffix_style: "dimmed",
            change_hash_length: 7,
            commit_hash_length: 7,
            disabled: false,
        }
    }
}
