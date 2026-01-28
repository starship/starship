use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct LeanConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl Default for LeanConfig<'_> {
    fn default() -> Self {
        Self {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: " L∃∀N ",
            style: "blue",
            disabled: false,
            detect_extensions: vec!["lean"],
            detect_files: vec![
                "lake-manifest.json",
                "lakefile.toml",
                "lakefile.lean",
                "lean-toolchain",
            ],
            detect_folders: vec![".lake"],
        }
    }
}
