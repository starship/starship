use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "config-schema", derive(schemars::JsonSchema))]
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
            style: "cyan bold",
            symbol: "C++ ",
            disabled: false,
            detect_extensions: vec!["cpp", "hpp"],
            detect_files: vec![],
            detect_folders: vec![],
            commands: vec![
                vec!["c++", "--version"],
                vec!["cpp", "--version"],
                vec!["g++", "--version"],
                vec!["clang++", "--version"],
            ],
        }
    }
}
