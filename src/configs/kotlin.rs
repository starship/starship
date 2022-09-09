use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct KotlinConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub kotlin_binary: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for KotlinConfig<'a> {
    fn default() -> Self {
        KotlinConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "🅺 ",
            style: "bold blue",
            kotlin_binary: "kotlin",
            disabled: false,
            detect_extensions: vec!["kt", "kts"],
            detect_files: vec![],
            detect_folders: vec![],
        }
    }
}
