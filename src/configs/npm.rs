use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct NpmConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub not_capable_style: &'a str,
    pub detect_files: Vec<&'a str>,
}

impl<'a> Default for NpmConfig<'a> {
    fn default() -> Self {
        NpmConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: " ",
            style: "bold 88",
            disabled: true,
            not_capable_style: "bold red",
            detect_files: vec!["package-lock.json", ".npmrc"],
        }
    }
}
