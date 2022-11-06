use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct SolidityConfig<'a> {
    pub format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub version_format: &'a str,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}
impl<'a> Default for SolidityConfig<'a> {
    fn default() -> Self {
        SolidityConfig {
            format: "via [$symbol($version)]($style)",
            symbol: "S ",
            style: "bold blue",
            disabled: false,
            version_format: "v${major}.${minor}.${patch}",
            detect_extensions: vec!["sol"],
            detect_files: vec![],
            detect_folders: vec![],
        }
    }
}
