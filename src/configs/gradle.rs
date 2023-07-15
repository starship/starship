use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GradleConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub recursive: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for GradleConfig<'a> {
    fn default() -> Self {
        GradleConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "🅶 ",
            style: "bold bright-cyan",
            disabled: false,
            recursive: false,
            detect_extensions: vec!["gradle", "gradle.kts"],
            detect_files: vec![],
            detect_folders: vec!["gradle"],
        }
    }
}
