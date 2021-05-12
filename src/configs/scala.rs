use crate::config::ModuleConfig;

use serde::Serialize;
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig, Serialize)]
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
