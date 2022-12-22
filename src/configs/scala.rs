use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct ScalaConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub disabled: bool,
    pub style: &'a str,
    pub symbol: &'a str,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for ScalaConfig<'a> {
    fn default() -> Self {
        ScalaConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            disabled: false,
            style: "red bold",
            symbol: "🆂 ",
            detect_extensions: vec!["sbt", "scala"],
            detect_files: vec![".scalaenv", ".sbtenv", "build.sbt"],
            detect_folders: vec![".metals"],
        }
    }
}
