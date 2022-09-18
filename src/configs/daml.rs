use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct DamlConfig<'a> {
    pub symbol: &'a str,
    pub format: &'a str,
    pub version_format: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for DamlConfig<'a> {
    fn default() -> Self {
        DamlConfig {
            symbol: "Λ ",
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            style: "bold cyan",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["daml.yaml"],
            detect_folders: vec![],
        }
    }
}
