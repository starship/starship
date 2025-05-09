use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct FortranConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub commands: Vec<Vec<&'a str>>,
}

impl Default for FortranConfig<'_> {
    fn default() -> Self {
        FortranConfig {
            format: "via [$symbol($version(-$name) )]($style)",
            version_format: "${raw}",
            symbol: "🅵  ",
            style: "bold purple",
            disabled: false,
            detect_extensions: vec![
                "f", "F", "for", "FOR", "ftn", "FTN", "f77", "F77", "f90", "F90", "f95", "F95",
                "f03", "F03", "f08", "F08", "f18", "F18",
            ],
            detect_files: vec!["fpm.toml"],
            detect_folders: vec![],
            commands: vec![
                // first check for most common 'gfortran' by default
                vec!["gfortran", "--version"],
                vec!["flang", "--version"],
                vec!["flang-new", "--version"],
            ],
        }
    }
}
