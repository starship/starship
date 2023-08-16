use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct HaskellConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for HaskellConfig<'a> {
    fn default() -> Self {
        HaskellConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "λ ",
            style: "bold purple",
            disabled: false,
            detect_extensions: vec!["hs", "cabal", "hs-boot"],
            detect_files: vec!["stack.yaml", "cabal.project"],
            detect_folders: vec![],
        }
    }
}
