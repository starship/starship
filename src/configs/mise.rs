use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct MiseConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub healthy_symbol: &'a str,
    pub unhealthy_symbol: &'a str,
}

impl Default for MiseConfig<'_> {
    fn default() -> Self {
        Self {
            format: "on [$symbol$health]($style) ",
            symbol: "mise ",
            style: "bold purple",
            disabled: true,
            detect_extensions: vec![],
            detect_files: vec![
                "mise.toml",
                "mise.local.toml",
                ".mise.toml",
                ".mise.local.toml",
            ],
            detect_folders: vec![".mise"],
            healthy_symbol: "healthy",
            unhealthy_symbol: "unhealthy",
        }
    }
}
