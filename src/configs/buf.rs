use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct BufConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for BufConfig<'a> {
    fn default() -> Self {
        BufConfig {
            format: "with [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "🐃 ",
            style: "bold blue",
            disabled: false,
            detect_extensions: vec![],
            detect_files: vec!["buf.yaml", "buf.gen.yaml", "buf.work.yaml"],
            detect_folders: vec![],
        }
    }
}
