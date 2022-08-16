use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct TerraformConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for TerraformConfig<'a> {
    fn default() -> Self {
        TerraformConfig {
            format: "via [$symbol$workspace]($style) ",
            version_format: "v${raw}",
            symbol: "💠 ",
            style: "bold 105",
            disabled: false,
            detect_extensions: vec!["tf", "tfplan", "tfstate"],
            detect_files: vec![],
            detect_folders: vec![".terraform"],
        }
    }
}
