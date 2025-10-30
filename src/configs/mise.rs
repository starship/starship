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
    pub style_missing_all: &'a str,
    pub style_missing_some: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub local_only: bool,
}

impl Default for MiseConfig<'_> {
    fn default() -> Self {
        Self {
            format: "with [$symbol$installed/$required]($style) ",
            symbol: "💾 mise ",
            style: "bold purple",
            style_missing_some: "bold yellow",
            style_missing_all: "bold red",
            disabled: true,
            detect_extensions: vec![],
            detect_files: vec![
                "mise.toml",
                "mise.local.toml",
                ".mise.toml",
                ".mise.local.toml",
            ],
            detect_folders: vec![".mise"],
            local_only: true,
        }
    }
}
