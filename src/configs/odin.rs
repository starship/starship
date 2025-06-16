use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct OdinConfig<'a> {
    pub format: &'a str,
    pub show_commit: bool,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl Default for OdinConfig<'_> {
    fn default() -> Self {
        OdinConfig {
            format: "via [$symbol($version )]($style)",
            show_commit: false,
            symbol: "Ø ",
            style: "bold bright-blue",
            disabled: false,
            detect_extensions: vec!["odin"],
            detect_files: vec![],
            detect_folders: vec![],
        }
    }
}
