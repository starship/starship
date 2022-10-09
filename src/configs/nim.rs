use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct NimConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for NimConfig<'a> {
    fn default() -> Self {
        NimConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "👑 ",
            style: "yellow bold",
            disabled: false,
            detect_extensions: vec!["nim", "nims", "nimble"],
            detect_files: vec!["nim.cfg"],
            detect_folders: vec![],
        }
    }
}
