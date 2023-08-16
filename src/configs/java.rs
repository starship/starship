use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct JavaConfig<'a> {
    pub disabled: bool,
    pub format: &'a str,
    pub version_format: &'a str,
    pub style: &'a str,
    pub symbol: &'a str,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for JavaConfig<'a> {
    fn default() -> Self {
        JavaConfig {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            disabled: false,
            style: "red dimmed",
            symbol: "☕ ",
            detect_extensions: vec!["java", "class", "jar", "gradle", "clj", "cljc"],
            detect_files: vec![
                "pom.xml",
                "build.gradle.kts",
                "build.sbt",
                ".java-version",
                "deps.edn",
                "project.clj",
                "build.boot",
                ".sdkmanrc",
            ],
            detect_folders: vec![],
        }
    }
}
