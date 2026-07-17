use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JJBookmarkConfig<'a> {
    /// Format string for the module
    pub format: &'a str,
    /// `$symbol` in the module's format
    pub symbol: &'a str,
    /// `$style` in the module's format
    pub style: &'a str,

    /// Truncate bookmark names and remotes if they're longer
    pub truncation_length: u16,
    /// Symbol used to signify truncation was used
    pub truncation_symbol: &'a str,

    /// Symbol added at the end of a bookmark's string if it has diverged from its remote state
    pub diverged_symbol: &'a str,

    /// Ignore bookmarks with this name
    ///
    /// Useful to filter out items like `main`, `master` or `develop` that are often default
    /// bookmarks you don't advance manually.
    pub ignore_names: Vec<&'a str>,
    /// Ignore bookmarks with this remote
    ///
    /// Useful to filter out items like `main@upstream` when you develop in a fork.
    pub ignore_remotes: Vec<&'a str>,

    /// Disable the module
    pub disabled: bool,
}

impl Default for JJBookmarkConfig<'_> {
    fn default() -> Self {
        Self {
            format: "on [$symbol$bookmark( \\(+$overflow_count others\\))]($style) ",
            symbol: " ",
            style: "bold purple",
            truncation_length: u16::MAX,
            truncation_symbol: "…",
            diverged_symbol: "*",
            ignore_names: vec![],
            ignore_remotes: vec![],
            disabled: false,
        }
    }
}
