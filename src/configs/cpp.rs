use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct CppConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub commands: Vec<Vec<&'a str>>,
}

impl<'a> Default for CppConfig<'a> {
    fn default() -> Self {
        CppConfig {
            format: "via [$symbol($version(-$name) )]($style)",
            version_format: "v${raw}",
            style: "149 bold",
            symbol: "C++ ",
            disabled: false,
            detect_extensions: vec!["cc", "cpp", "cxx", "hpp"],
            detect_files: vec![],
            detect_folders: vec![],
            commands: vec![
                vec!["g++", "--version"],
                vec!["clang", "--version"]
            ],
        }
    }
}
