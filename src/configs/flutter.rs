use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct FlutterConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub check_pubspec: bool,
}

impl<'a> Default for FlutterConfig<'a> {
    fn default() -> Self {
        FlutterConfig {
            format: "via [$symbol($version \\($channel\\) )]($style)",
            version_format: "v${raw}",
            symbol: "🐦 ",
            style: "bold blue",
            disabled: false,
            detect_extensions: vec!["dart"],
            detect_files: vec!["pubspec.yaml", "pubspec.lock"],
            detect_folders: vec![".dart_tool"],
            check_pubspec: true,
        }
    }
}
