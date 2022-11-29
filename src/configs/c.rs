use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct CConfig<'a> {
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

impl<'a> Default for CConfig<'a> {
    fn default() -> Self {
        CConfig {
            format: "via [$symbol($version(-$name) )]($style)",
            version_format: "v${raw}",
            style: "bold green",
            symbol: " ",
            disabled: false,
            detect_extensions: vec!["c", "h"],
            detect_files: vec![],
            detect_folders: vec![],
            commands: vec![
                // the compiler is usually cc, and --version works on gcc and clang
                vec!["cc", "--version"],
                // but on some platforms gcc is installed as *gcc*, not cc
                vec!["gcc", "--version"],
                // for completeness, although I've never seen a clang that wasn't cc
                vec!["clang", "--version"],
            ],
        }
    }
}
