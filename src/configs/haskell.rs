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
    pub default_compiler: &'a str,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub cabal_extensions: Vec<&'a str>,
    pub cabal_files: Vec<&'a str>,
    pub cabal_folders: Vec<&'a str>,
    pub stack_extensions: Vec<&'a str>,
    pub stack_files: Vec<&'a str>,
    pub stack_folders: Vec<&'a str>,
}

impl Default for HaskellConfig<'_> {
    fn default() -> Self {
        Self {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "λ ",
            style: "bold purple",
            disabled: false,
            default_compiler: "ghc",
            detect_extensions: vec!["hs", "hs-boot"],
            detect_files: vec![],
            detect_folders: vec![],
            cabal_extensions: vec!["cabal"],
            cabal_files: vec!["cabal.project"],
            cabal_folders: vec![],
            stack_extensions: vec![],
            stack_files: vec!["stack.yaml"],
            stack_folders: vec![],
        }
    }
}
