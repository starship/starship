use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct HaxeConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for HaxeConfig<'a> {
    fn default() -> Self {
        HaxeConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "⌘ ",
            style: "bold fg:202",
            disabled: false,
            detect_extensions: vec!["hx", "hxml"],
            detect_files: vec![
                "haxelib.json",
                "hxformat.json",
                ".haxerc",
            ],
            detect_folders: vec![".haxelib", "haxe_libraries"],
        }
    }
}
