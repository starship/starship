use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct SpinConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub permanent: bool,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for SpinConfig<'a> {
    fn default() -> Self {
        SpinConfig {
            format: "[$symbol($version) ]($style)",
            version_format: "Spin ${raw}",
            symbol: "💫 ",
            permanent: false,
            style: "bold bright-purple",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["spin.toml", "Spin.toml"],
            detect_folders: vec![".spin"],
        }
    }
}
