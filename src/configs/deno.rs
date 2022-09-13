use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct DenoConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for DenoConfig<'a> {
    fn default() -> Self {
        DenoConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "🦕 ",
            style: "green bold",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec![
                "deno.json",
                "deno.jsonc",
                "mod.ts",
                "deps.ts",
                "mod.js",
                "deps.js",
            ],
            detect_folders: vec![],
        }
    }
}
