use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct MoonBitConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for MoonBitConfig<'a> {
    fn default() -> Self {
        MoonBitConfig {
            format: "via [$symbol($version )]($style)",
            symbol: "🐰 ",
            style: "bold purple",
            disabled: false,
            detect_extensions: vec!["mbt", "mbti"],
            detect_files: vec!["moon.mod.json", "moon.pkg.json"],
            detect_folders: vec![],
        }
    }
}
