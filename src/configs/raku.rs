use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct RakuConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for RakuConfig<'a> {
    fn default() -> Self {
        RakuConfig {
            format: "via [$symbol($version-$vm_version )]($style)",
            version_format: "${raw}",
            symbol: "🦋 ",
            style: "149 bold",
            disabled: false,
            detect_extensions: vec!["p6", "pm6", "pod6", "raku", "rakumod"],
            detect_files: vec!["META6.json"],
            detect_folders: vec![],
        }
    }
}
