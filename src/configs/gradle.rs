use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct GradleConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
    pub strategy: GradleVersionStrategy,
}

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(try_from = "String")]
pub enum GradleVersionStrategy {
    #[serde(rename = "properties")]
    WrapperProperties,
    #[serde(rename = "executable")]
    Executable,
}

pub struct InvalidGradleVersionStrategy(String);

impl Display for InvalidGradleVersionStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is not a valid gradle version strategy. Available strategies are: `properties` and `executable`", self.0)
    }
}

impl TryFrom<String> for GradleVersionStrategy {
    type Error = InvalidGradleVersionStrategy;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let strategy = match value.as_str() {
            "properties" => GradleVersionStrategy::WrapperProperties,
            "executable" => GradleVersionStrategy::Executable,
            _ => return Err(InvalidGradleVersionStrategy(value)),
        };
        Ok(strategy)
    }
}

impl<'a> Default for GradleConfig<'a> {
    fn default() -> Self {
        GradleConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "🅶 ",
            style: "bold bright-cyan",
            disabled: false,
            detect_extensions: vec!["gradle", "gradle.kts"],
            detect_files: vec![],
            detect_folders: vec![".gradle"],
            strategy: GradleVersionStrategy::WrapperProperties,
        }
    }
}
